# Contribuire a Sedette

Prima di tutto leggi la specifica in [docs/PROMPT.md](docs/PROMPT.md) e i vincoli in [SECURITY.md](SECURITY.md).

## Flusso di lavoro

Tutto il lavoro passa da GitHub: **ogni attività è una issue, ogni modifica una pull request** collegata alla sua issue.

1. **Issue.** Aprila con uno dei form (`feature`, `bug`, `design-decision`, `research`). Aggiungi le label `fase:*`, `area:*`,
   `prio:*` e la milestone della fase. Le issue di una fase si creano dopo che il piano della fase è approvato.
2. **Branch.** Un branch per issue, partendo da `main` aggiornato: `<tipo>/<numero>-<nome-breve>`, per esempio
   `feat/11-ingestor-blocchi`, `fix/42-decodifica-fee`, `docs/4-documentazione-base`. I tipi sono quelli dei commit.
3. **Commit.** Formato [Conventional Commits](https://www.conventionalcommits.org/it/v1.0.0/): `feat:`, `fix:`, `docs:`,
   `refactor:`, `test:`, `chore:`, `ci:`, `perf:`. Il changelog si genera da questi messaggi.
4. **Pull request.** Il titolo segue Conventional Commits, perché diventa il messaggio del commit squash. La descrizione
   contiene `Closes #N` e la checklist del template.
5. **Merge.** Solo squash, solo con CI verde. Il branch viene cancellato automaticamente. Su `main` non si fa push diretto:
   il ruleset lo impedisce.

Stati della board: **Backlog** (non pianificata) → **Design** (in attesa di una decisione) → **Pronto** (approvata) →
**In corso** (branch aperto) → **Review** (PR aperta) → **Fatto** (PR unita).

## Fasi, checkpoint e release

- Il lavoro procede **per fasi** (milestone `Fase N – …`). A fine fase si scrive un riepilogo (cosa è stato fatto, cosa manca,
  come è stato verificato) e ci si ferma fino al via per la fase successiva.
- Dopo le fasi 1, 2 e 4 c'è un **checkpoint di design**: un documento in [docs/design/](docs/design/README.md) e, per ogni
  decisione presa, un **ADR** in [docs/adr/](docs/adr/README.md). La fase successiva si pianifica solo dopo che l'ADR è
  fuso in `main`.
- Ogni fase si chiude con un **tag** (`v0.1.0` per la Fase 0, `v0.2.0` per la Fase 1, …) e con `CHANGELOG.md` generato da
  [git-cliff](https://git-cliff.org/). L'aggiornamento del changelog passa da una PR `chore(release): vX.Y.Z`; il tag si crea
  sul commit di merge.

## Qualità del codice

- `cargo fmt`, `cargo clippy -- -D warnings`, test e `cargo deny check` devono passare a ogni PR.
- Log con `tracing`; errori con `thiserror` nelle librerie e `anyhow` nei binari.
- Segreti (URL RPC con chiave, password) **solo** come variabili d'ambiente, mai nel codice o nei file versionati.
- Ogni crate esterno oltre a quelli previsti dalla specifica va motivato nella PR che lo introduce.
- Niente look-ahead e niente survivorship bias: ogni decisione simulata usa solo i dati disponibili in quel momento.
  Dichiara sempre la dimensione del campione e l'incertezza delle metriche.

## Politica dei dati

Il repository è pubblico: niente dati grezzi o derivati, niente export, niente elenchi di wallet classificati o marcati
come anomalia. Nei documenti vanno solo aggregati e metodologia. Dettagli in [SECURITY.md](SECURITY.md#politica-dei-dati).

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
