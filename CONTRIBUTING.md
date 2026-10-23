# Contribuire a Sedette

## Struttura GitHub

La configurazione del repository è descritta in codice nello script
[`scripts/github/bootstrap.sh`](scripts/github/bootstrap.sh). Lo script è idempotente: rieseguirlo su un repository già
configurato non cambia nulla, e con `--dry-run` mostra le differenze senza scrivere.

```bash
# richiede gh autenticato con gli scope `repo` e `project` e i permessi di amministratore
gh auth refresh -s project
bash scripts/github/bootstrap.sh --dry-run
bash scripts/github/bootstrap.sh
```

Lo script gestisce:

| Ambito | Configurazione |
|---|---|
| Sicurezza | Secret scanning e push protection, Dependabot alerts e security updates, segnalazione privata delle vulnerabilità. |
| Merge | Solo squash; il titolo della PR diventa il titolo del commit; il branch viene cancellato dopo il merge. |
| Actions | `GITHUB_TOKEN` in sola lettura di default; le Actions non possono approvare PR; le PR da fork esterni richiedono approvazione. |
| Ruleset `main` | Nessuna cancellazione, nessun force push, storia lineare, PR obbligatoria con merge squash, nessun bypass. I check di CI obbligatori sono elencati in `REQUIRED_CHECKS`. |
| Ruleset `release-tags` | I tag `v*` non si cancellano e non si spostano. |
| Label | `fase:0`…`fase:5`, `type:*`, `area:*`, `prio:*`. |
| Milestone | Una per fase e una per ogni checkpoint di design. |
| Board | Board Projects "Sedette", pubblica e collegata al repository, con le colonne Backlog → Design → Pronto → In corso → Review → Fatto e una vista board. |

### Passaggi manuali

Alcune impostazioni non sono disponibili via API e vanno fatte dall'interfaccia web.

1. **Automazioni della board.** Apri la board → menu `⋯` → *Workflows* e attiva:
   - *Auto-add to project*: repository `Sedette`, filtro `is:issue,pr is:open`. Il piano Free permette un solo workflow di questo tipo.
   - *Item added to project* → Status `Backlog`.
   - *Item reopened* → Status `Backlog`.
   - *Item closed* → Status `Fatto`.
   - *Pull request merged* → Status `Fatto`.

   I workflow predefiniti puntavano allo stato `Done`, che non esiste più: va riselezionato `Fatto`.
2. **Email dell'account.** In *Settings → Emails* tieni attivi "Keep my email addresses private" e "Block command line pushes
   that expose my email". I commit devono usare l'indirizzo `…@users.noreply.github.com`.
