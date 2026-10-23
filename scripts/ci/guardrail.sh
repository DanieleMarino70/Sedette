#!/usr/bin/env bash
# Controlli dei vincoli non negoziabili di Sedette (vedi SECURITY.md).
# Analizza solo i file versionati. Esce con codice 1 se trova una violazione
# e con codice 2 se un controllo non riesce a girare.
#
#  1. Nessun riferimento ai domini di Polymarket nel codice, nella
#     configurazione, nell'infrastruttura e nei workflow.
#  2. Nessuna feature o crate di firma (signer) di alloy.
#  3. Nessun file .env versionato, tranne .env.example.
#  4. Nessun URL RPC con chiave nei file versionati.
set -euo pipefail

cd "$(git rev-parse --show-toplevel)"
FAILED=0

# scan <messaggio> <argomenti di git grep...>
# Segnala una violazione se git grep trova occorrenze; si ferma se git grep
# fallisce (codice > 1), così un errore non passa per "nessuna occorrenza".
scan() {
  local msg="$1" out rc=0
  shift
  out=$(git grep "$@") || rc=$?
  case "$rc" in
    0) echo "::error::$msg"; printf '%s\n' "$out"; FAILED=1 ;;
    1) ;;
    *) echo "::error::git grep non riuscito (codice $rc) durante il controllo: $msg"; exit 2 ;;
  esac
}

# 1. Domini di Polymarket. La documentazione (*.md) può citarli per spiegare
#    il vincolo; il codice e la configurazione no.
paths=()
for p in crates bins infra scripts compose.yaml sedette.toml Cargo.toml .github/workflows .env.example; do
  [ -e "$p" ] && paths+=("$p")
done
scan "riferimenti ai domini di Polymarket" -nIiE 'polymarket\.com' -- "${paths[@]}" ':!scripts/ci/guardrail.sh'

# 2. Signer di alloy: nelle feature dei manifest e nel lockfile.
scan "feature o dipendenze di firma nei Cargo.toml" -nE 'signer' -- 'Cargo.toml' '**/Cargo.toml'
scan "crate signer di alloy nel Cargo.lock" -nE '^name = "alloy-signer-[a-z0-9-]+"' -- 'Cargo.lock'

# 3. File .env versionati.
if envs=$(git ls-files | grep -E '(^|/)\.env($|\.)' | grep -vE '(^|/)\.env\.example$'); then
  echo "::error::file .env versionati (i segreti stanno solo nel .env locale)"
  printf '%s\n' "$envs"
  FAILED=1
fi

# 4. URL RPC con chiave, nei formati dei provider più comuni.
rpc_key='infura\.io/(ws/)?v3/[0-9a-fA-F]{32}'
rpc_key+='|alchemy\.com/v2/[A-Za-z0-9_-]{20,}'
rpc_key+='|quiknode\.pro/[0-9a-fA-F]{20,}'
rpc_key+='|p2pify\.com/[0-9a-fA-F]{32}'
rpc_key+='|drpc\.org/[^[:space:]]*[?&]dkey=[A-Za-z0-9_-]{20,}'
rpc_key+='|rpc\.ankr\.com/[a-z_]+/[0-9a-fA-F]{64}'
scan "possibili URL RPC con chiave: revoca la chiave e rimuovi il file" -nIE "$rpc_key" -- . ':!scripts/ci/guardrail.sh'

if [ "$FAILED" = 0 ]; then
  echo "guardrail: nessuna violazione"
fi
exit "$FAILED"
