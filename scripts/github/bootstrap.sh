#!/usr/bin/env bash
# Configura il repository GitHub di Sedette in modo idempotente:
# sicurezza, impostazioni di merge, permessi delle Actions, ruleset,
# label, milestone e board Projects.
#
# Uso: scripts/github/bootstrap.sh [--dry-run] [--repo OWNER/NOME]
#
# Richiede `gh` autenticato con gli scope `repo` e `project` e i permessi
# di amministratore sul repository. Non richiede jq: i confronti usano il
# filtro --jq integrato in gh. Rieseguirlo su un repo già configurato non
# cambia nulla. Con --dry-run non fa scritture.
set -euo pipefail

DRY_RUN=0
REPO="DanieleMarino70/Sedette"
PROJECT_TITLE="Sedette"
PROJECT_DESCRIPTION="Sedette: motore di analisi dei mercati predittivi on-chain (sola lettura) con conto demo"

# Check obbligatori su main: i nomi dei job di .github/workflows/ci.yml.
# Un check richiesto ma mai eseguito bloccherebbe ogni merge: aggiungi un nome
# qui solo dopo che il job relativo è su main.
REQUIRED_CHECKS=(fmt clippy test deny guardrail compose pr-title)

# Richiede i risultati di CodeQL (default setup) prima del merge su main.
REQUIRE_CODE_SCANNING=1

STATUS_OPTIONS="Backlog,Design,Pronto,In corso,Review,Fatto"

while [ $# -gt 0 ]; do
  case "$1" in
    --dry-run) DRY_RUN=1 ;;
    --repo) REPO="$2"; shift ;;
    -h | --help) sed -n '2,11p' "$0"; exit 0 ;;
    *) echo "argomento sconosciuto: $1" >&2; exit 2 ;;
  esac
  shift
done

OWNER="${REPO%%/*}"
NAME="${REPO##*/}"
R="repos/$REPO"
CHANGES=0

section() { printf '\n== %s\n' "$*"; }
ok() { printf '  = %s\n' "$*"; }
change() {
  CHANGES=$((CHANGES + 1))
  if [ "$DRY_RUN" = 1 ]; then printf '  ~ [dry-run] %s\n' "$*"; else printf '  ~ %s\n' "$*"; fi
}
# Esegue il comando solo fuori dal dry-run, scartando l'output.
run() {
  if [ "$DRY_RUN" = 1 ]; then return 0; fi
  "$@" >/dev/null
}

# --- Sicurezza ---------------------------------------------------------------
section "Sicurezza"

actual=$(gh api "$R" --jq '.security_and_analysis | "\(.secret_scanning.status) \(.secret_scanning_push_protection.status)"')
if [ "$actual" = "enabled enabled" ]; then
  ok "secret scanning e push protection attivi"
else
  change "attivo secret scanning e push protection (stato: $actual)"
  run gh api -X PATCH "$R" --input - <<'EOF'
{"security_and_analysis":{"secret_scanning":{"status":"enabled"},"secret_scanning_push_protection":{"status":"enabled"}}}
EOF
fi

if gh api "$R/vulnerability-alerts" --silent 2>/dev/null; then
  ok "Dependabot alerts attivi"
else
  change "attivo Dependabot alerts"
  run gh api -X PUT "$R/vulnerability-alerts"
fi

if [ "$(gh api "$R/automated-security-fixes" --jq .enabled 2>/dev/null || echo false)" = "true" ]; then
  ok "Dependabot security updates attivi"
else
  change "attivo Dependabot security updates"
  run gh api -X PUT "$R/automated-security-fixes"
fi

if [ "$(gh api "$R/private-vulnerability-reporting" --jq .enabled)" = "true" ]; then
  ok "segnalazione privata delle vulnerabilità attiva"
else
  change "attivo la segnalazione privata delle vulnerabilità"
  run gh api -X PUT "$R/private-vulnerability-reporting"
fi

# CodeQL in default setup: le lingue (Rust, GitHub Actions) sono rilevate da GitHub.
actual=$(gh api "$R/code-scanning/default-setup" --jq '"\(.state) \(.languages | join(","))"')
if [ "${actual%% *}" = "configured" ]; then
  ok "CodeQL default setup attivo (${actual#* })"
else
  change "attivo CodeQL in default setup (stato: $actual)"
  run gh api -X PATCH "$R/code-scanning/default-setup" -f state=configured -f query_suite=default
fi

# --- Merge -------------------------------------------------------------------
section "Impostazioni di merge"

want="false false true true PR_TITLE COMMIT_MESSAGES true false"
actual=$(gh api "$R" --jq '"\(.allow_merge_commit) \(.allow_rebase_merge) \(.allow_squash_merge) \(.delete_branch_on_merge) \(.squash_merge_commit_title) \(.squash_merge_commit_message) \(.allow_update_branch) \(.has_wiki)"')
if [ "$actual" = "$want" ]; then
  ok "solo squash, titolo della PR, branch cancellato dopo il merge"
else
  change "allineo le impostazioni di merge (stato: $actual)"
  run gh api -X PATCH "$R" --input - <<'EOF'
{"allow_squash_merge":true,"allow_merge_commit":false,"allow_rebase_merge":false,"delete_branch_on_merge":true,"squash_merge_commit_title":"PR_TITLE","squash_merge_commit_message":"COMMIT_MESSAGES","allow_update_branch":true,"has_wiki":false}
EOF
fi

# --- Actions -----------------------------------------------------------------
section "Permessi delle Actions"

actual=$(gh api "$R/actions/permissions/workflow" --jq '"\(.default_workflow_permissions) \(.can_approve_pull_request_reviews)"')
if [ "$actual" = "read false" ]; then
  ok "GITHUB_TOKEN in sola lettura, nessuna approvazione di PR dalle Actions"
else
  change "imposto GITHUB_TOKEN in sola lettura (stato: $actual)"
  run gh api -X PUT "$R/actions/permissions/workflow" \
    -f default_workflow_permissions=read -F can_approve_pull_request_reviews=false
fi

actual=$(gh api "$R/actions/permissions/fork-pr-contributor-approval" --jq .approval_policy)
if [ "$actual" = "all_external_contributors" ]; then
  ok "approvazione obbligatoria per le PR da fork esterni"
else
  change "richiedo l'approvazione per le PR da fork esterni (stato: $actual)"
  run gh api -X PUT "$R/actions/permissions/fork-pr-contributor-approval" \
    -f approval_policy=all_external_contributors
fi

# --- Ruleset -----------------------------------------------------------------
section "Ruleset"

# Riassunto canonico di un ruleset, per capire se un PUT ha cambiato qualcosa.
ruleset_summary() {
  gh api "$R/rulesets/$1" --jq '[.enforcement, .target, (.conditions | tojson), (.bypass_actors | tojson),
    ([.rules[] | .type + "=" + ((.parameters // {}) | tojson)] | sort | join(";"))] | join("|")'
}

# Crea o aggiorna il ruleset $1 con il JSON letto da stdin.
upsert_ruleset() {
  local name="$1" body id before after
  body=$(cat)
  id=$(gh api "$R/rulesets" --jq ".[] | select(.name == \"$name\") | .id")
  if [ -z "$id" ]; then
    change "creo il ruleset $name"
    if [ "$DRY_RUN" = 0 ]; then printf '%s' "$body" | gh api -X POST "$R/rulesets" --input - >/dev/null; fi
    return
  fi
  if [ "$DRY_RUN" = 1 ]; then
    ok "ruleset $name presente (in dry-run il contenuto non viene confrontato)"
    return
  fi
  before=$(ruleset_summary "$id")
  printf '%s' "$body" | gh api -X PUT "$R/rulesets/$id" --input - >/dev/null
  after=$(ruleset_summary "$id")
  if [ "$before" = "$after" ]; then ok "ruleset $name invariato"; else change "ruleset $name aggiornato"; fi
}

status_checks_rule=""
if [ "${#REQUIRED_CHECKS[@]}" -gt 0 ]; then
  checks=""
  for c in "${REQUIRED_CHECKS[@]}"; do
    # integration_id 15368 = GitHub Actions: il check deve arrivare dalle Actions.
    checks+="${checks:+,}{\"context\":\"$c\",\"integration_id\":15368}"
  done
  status_checks_rule=",{\"type\":\"required_status_checks\",\"parameters\":{\"strict_required_status_checks_policy\":true,\"do_not_enforce_on_create\":false,\"required_status_checks\":[$checks]}}"
fi

code_scanning_rule=""
if [ "$REQUIRE_CODE_SCANNING" = 1 ]; then
  code_scanning_rule=',{"type":"code_scanning","parameters":{"code_scanning_tools":[{"tool":"CodeQL","security_alerts_threshold":"high_or_higher","alerts_threshold":"errors"}]}}'
fi

upsert_ruleset main <<EOF
{
  "name": "main",
  "target": "branch",
  "enforcement": "active",
  "bypass_actors": [],
  "conditions": {"ref_name": {"include": ["~DEFAULT_BRANCH"], "exclude": []}},
  "rules": [
    {"type": "deletion"},
    {"type": "non_fast_forward"},
    {"type": "required_linear_history"},
    {"type": "pull_request", "parameters": {
      "required_approving_review_count": 0,
      "dismiss_stale_reviews_on_push": false,
      "require_code_owner_review": false,
      "require_last_push_approval": false,
      "required_review_thread_resolution": false,
      "allowed_merge_methods": ["squash"]
    }}$status_checks_rule$code_scanning_rule
  ]
}
EOF

upsert_ruleset release-tags <<'EOF'
{
  "name": "release-tags",
  "target": "tag",
  "enforcement": "active",
  "bypass_actors": [],
  "conditions": {"ref_name": {"include": ["refs/tags/v*"], "exclude": []}},
  "rules": [
    {"type": "deletion"},
    {"type": "non_fast_forward"},
    {"type": "update"}
  ]
}
EOF

# --- Label -------------------------------------------------------------------
section "Label"

while IFS='|' read -r label color desc; do
  [ -z "$label" ] && continue
  actual=$(gh api "$R/labels/$label" --jq '"\(.color)|\(.description)"' 2>/dev/null || true)
  if [ -z "$actual" ]; then
    change "creo la label $label"
    run gh api -X POST "$R/labels" -f name="$label" -f color="$color" -f description="$desc"
  elif [ "${actual,,}" != "${color,,}|${desc,,}" ]; then
    change "aggiorno la label $label"
    run gh api -X PATCH "$R/labels/$label" -f color="$color" -f description="$desc"
  else
    ok "$label"
  fi
done <<'EOF'
fase:0|D4C5F9|Fase 0 – Scheletro
fase:1|BFA8F5|Fase 1 – Ingestione on-chain
fase:2|A78BF0|Fase 2 – Analisi dei wallet
fase:3|8E6CEB|Fase 3 – Segnali
fase:4|754DE6|Fase 4 – Conto demo (paper trading)
fase:5|5319E7|Fase 5 – Interfaccia
type:feature|1D76DB|Nuova funzionalità
type:bug|D73A4A|Comportamento errato da correggere
type:design|E99695|Checkpoint di design o ADR
type:research|FBCA04|Verifica su fonti ufficiali (contratti, commissioni, dati)
type:chore|BFD4F2|Manutenzione, infrastruttura, CI, documentazione
area:chain|0E8A16|Client chain, eventi, reorg e finalità
area:ingest|2DA44E|Backfill, streaming, data source
area:analytics|006B75|Posizioni, metriche, archetipi, anomalie
area:signals|0B7285|Segnali e panieri
area:paper|1F6FEB|Conto demo e backtest
area:infra|6E7781|Repo, CI, docker, configurazione, docs
area:ui|C2E0C6|Grafana, dashboard, notifiche
prio:alta|B60205|Priorità alta
prio:media|D93F0B|Priorità media
prio:bassa|FEF2C0|Priorità bassa
EOF

# --- Milestone ---------------------------------------------------------------
section "Milestone"

existing=$(gh api "$R/milestones?state=all&per_page=100" --jq '.[] | "\(.number)|\(.title)|\(.description)"')
while IFS='|' read -r title desc; do
  [ -z "$title" ] && continue
  line=$(printf '%s\n' "$existing" | awk -F'|' -v t="$title" '$2 == t' | head -n1)
  if [ -z "$line" ]; then
    change "creo la milestone $title"
    run gh api -X POST "$R/milestones" -f title="$title" -f description="$desc"
  elif [ "${line#*|*|}" != "$desc" ]; then
    change "aggiorno la descrizione di $title"
    run gh api -X PATCH "$R/milestones/${line%%|*}" -f description="$desc"
  else
    ok "$title"
  fi
done <<'EOF'
Fase 0 – Scheletro|Workspace, config, logging, docker-compose, CI, CONTRACTS.md, struttura GitHub. Release v0.1.0.
Fase 1 – Ingestione|Decodifica eventi on-chain, mappatura token→mercato, backfill, streaming, reorg e finalità. Release v0.2.0.
Checkpoint 1 – Design post-ingestione|Documento in docs/design/ e ADR delle scelte dopo la Fase 1.
Fase 2 – Analisi wallet|Posizioni, P&L, metriche, archetipi, test fortuna/abilità, anomalie. Release v0.3.0.
Checkpoint 2 – Design post-analisi|Documento in docs/design/ e ADR delle scelte dopo la Fase 2.
Fase 3 – Segnali|Consenso a panieri, segnale da singolo specialista, storico in Grafana. Release v0.4.0.
Fase 4 – Conto demo|Paper trading, modello di esecuzione prudente, rischio, backtest. Release v0.5.0.
Checkpoint 3 – Design interfaccia|Raccolta delle viste e 2-3 impostazioni complete della dashboard.
Fase 5 – Interfaccia|Dashboard Grafana del design scelto, bot Telegram opzionale solo notifiche. Release v0.6.0.
EOF

# --- Board Projects ----------------------------------------------------------
section "Board Projects"

num=$(gh project list --owner "$OWNER" --limit 100 --format json \
  --jq ".projects[] | select(.title == \"$PROJECT_TITLE\") | .number")
if [ -z "$num" ]; then
  change "creo la board $PROJECT_TITLE"
  if [ "$DRY_RUN" = 1 ]; then
    echo "  (dry-run: i controlli successivi sulla board vengono saltati)"
  else
    num=$(gh project create --owner "$OWNER" --title "$PROJECT_TITLE" --format json --jq .number)
  fi
else
  ok "board $PROJECT_TITLE (#$num)"
fi

if [ -n "$num" ]; then
  actual=$(gh project view "$num" --owner "$OWNER" --format json --jq '"\(.public)|\(.shortDescription)"')
  if [ "$actual" = "true|$PROJECT_DESCRIPTION" ]; then
    ok "board pubblica con descrizione"
  else
    change "rendo pubblica la board e ne imposto la descrizione"
    run gh project edit "$num" --owner "$OWNER" --visibility PUBLIC --description "$PROJECT_DESCRIPTION"
  fi

  linked=$(gh api graphql -f o="$OWNER" -F n="$num" \
    -f query='query($o: String!, $n: Int!) { user(login: $o) { projectV2(number: $n) { repositories(first: 50) { nodes { nameWithOwner } } } } }' \
    --jq '.data.user.projectV2.repositories.nodes[].nameWithOwner' | grep -Fx "$REPO" || true)
  if [ -n "$linked" ]; then
    ok "board collegata a $REPO"
  else
    change "collego la board a $REPO"
    run gh project link "$num" --owner "$OWNER" --repo "$NAME"
  fi

  read -r field_id actual < <(gh project field-list "$num" --owner "$OWNER" --format json \
    --jq '.fields[] | select(.name == "Status") | "\(.id) \([.options[].name] | join(","))"')
  if [ "$actual" = "$STATUS_OPTIONS" ]; then
    ok "colonne Status: $STATUS_OPTIONS"
  else
    # Attenzione: le opzioni vengono sostituite in blocco.
    change "imposto le colonne Status (stato: $actual)"
    run gh api graphql -f field="$field_id" -f query='
      mutation($field: ID!) {
        updateProjectV2Field(input: {
          fieldId: $field,
          singleSelectOptions: [
            {name: "Backlog",  color: GRAY,   description: "Idee e attività non ancora pianificate"},
            {name: "Design",   color: PURPLE, description: "In attesa di decisione di design o ADR"},
            {name: "Pronto",   color: BLUE,   description: "Pianificata e approvata, si può iniziare"},
            {name: "In corso", color: YELLOW, description: "Lavoro avviato su un branch"},
            {name: "Review",   color: ORANGE, description: "PR aperta in attesa di CI verde e merge"},
            {name: "Fatto",    color: GREEN,  description: "PR unita e issue chiusa"}
          ]
        }) { projectV2Field { ... on ProjectV2SingleSelectField { id } } }
      }'
  fi

  boards=$(gh api graphql -f o="$OWNER" -F n="$num" \
    -f query='query($o: String!, $n: Int!) { user(login: $o) { projectV2(number: $n) { views(first: 20) { nodes { name layout } } } } }' \
    --jq '[.data.user.projectV2.views.nodes[] | select(.layout == "BOARD_LAYOUT")] | length')
  if [ "$boards" -gt 0 ]; then
    ok "vista board presente"
  else
    change "creo la vista board raggruppata per Status"
    if [ "$DRY_RUN" = 0 ]; then
      status_id=$(gh api -H "X-GitHub-Api-Version: 2026-03-10" "users/$OWNER/projectsV2/$num/fields" \
        --jq '.[] | select(.name == "Status") | .id')
      gh api -H "X-GitHub-Api-Version: 2026-03-10" -X POST "users/$OWNER/projectsV2/$num/views" \
        --input - >/dev/null <<EOF
{"name": "Board", "layout": "board", "vertical_group_by": [$status_id]}
EOF
    fi
  fi
fi

# --- Riepilogo ---------------------------------------------------------------
printf '\n'
if [ "$CHANGES" = 0 ]; then
  echo "Nessuna modifica: il repository è già configurato."
elif [ "$DRY_RUN" = 1 ]; then
  echo "$CHANGES modifiche da applicare (dry-run, nulla è stato scritto)."
else
  echo "$CHANGES modifiche applicate."
fi
cat <<'EOF'

Passaggi manuali (non disponibili via API, vedi CONTRIBUTING.md):
  - automazioni della board dall'interfaccia web;
  - "Block command line pushes that expose my email" nelle impostazioni email dell'account.
EOF
