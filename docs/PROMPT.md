# Sedette: motore di analisi dei mercati predittivi in Rust con conto demo (sola lettura)

## Ruolo e contesto

Il progetto si chiama **Sedette** (repository `sedette`). Usa questo nome in README, documentazione, nomi dei crate, binari e configurazione.

Sei un senior Rust engineer con esperienza in dati on-chain EVM e in analisi quantitativa. Devi costruire con me, **fase per fase**, un motore di analisi dei mercati predittivi di Polymarket basato **esclusivamente su dati pubblici on-chain** (Polygon), con un **conto demo** (paper trading) che simula strategie con un bankroll virtuale.

Il progetto ha due obiettivi:
1. capire quali wallet hanno un vantaggio reale e misurabile, e separarli da fortuna, bot di spread e insider;
2. verificare con numeri onesti se una strategia (per esempio copiare panieri di specialisti) rende **dopo** ritardi, slippage e commissioni.

## Vincoli legali e di sicurezza (non negoziabili)

- **Nessuna operazione reale.** Il sistema non piazza ordini, non firma transazioni e non contiene codice per farlo. Nessuna private key, nessun wallet collegato.
- **Nessun accesso ai siti o alle API di Polymarket** (dominio polymarket.com e sottodomini) nel funzionamento predefinito. Nessun uso di VPN, proxy o tecniche per aggirare blocchi geografici. I dati arrivano dalla blockchain pubblica tramite un provider RPC di Polygon e, se utile, da fornitori di dati terzi i cui termini ne consentano l'uso (valutali e segnalameli prima di integrarli).
- Prevedi un'interfaccia `MarketDataSource` con un adapter per il CLOB di Polymarket **non implementato e disabilitato**, documentato come "da attivare solo in una giurisdizione in cui l'uso è legale". Serve solo a non dover riscrivere l'architettura in futuro.

## Regole di lavoro

1. **Lavora per fasi.** Alla fine di ogni fase fermati, riassumi cosa hai fatto, cosa manca e come l'ho verificato, e aspetta il mio via.
2. **Prima pianifica, poi scrivi codice.** Proponi il piano di ogni fase e segnala le decisioni che spettano a me.
3. **Non fidarti della memoria su contratti ed eventi.** Polymarket ha aggiornato più volte i contratti di exchange (esiste anche una versione v2). Ricava indirizzi, ABI ed eventi da fonti ufficiali (documentazione di Polymarket, repository GitHub ufficiali, contratti verificati su Polygonscan) e documenta la fonte di ognuno in un file `CONTRACTS.md`. Lo stesso vale per le API di alloy e delle altre librerie: verifica sulla documentazione.
4. **Rigore statistico.** Niente look-ahead bias (ogni decisione simulata usa solo dati disponibili in quel momento), niente survivorship bias (i wallet "forti" si selezionano con dati passati, non con il risultato finale). Dichiara sempre la dimensione del campione e l'incertezza delle metriche.
5. **Qualità:** `cargo fmt`, `cargo clippy -- -D warnings`, test che passano a ogni fase, `tracing` per i log, `thiserror` nelle librerie e `anyhow` nei binari. Segreti (URL RPC con chiave) solo via variabili d'ambiente, con `.env.example`.
6. **Checkpoint di design dopo le fasi importanti.** Al termine delle fasi marcate con 🎨 non si passa alla fase successiva finché non abbiamo fatto insieme il checkpoint di design descritto sotto.
7. **Tutto il lavoro passa da GitHub.** Ogni attività è una issue, ogni modifica una pull request collegata alla sua issue (vedi "Gestione del progetto su GitHub").

## Checkpoint di design

Dopo ogni fase importante (🎨: Fasi 1, 2 e 4) facciamo un checkpoint prima di proseguire. Tu prepari un documento in `docs/design/` con:

1. **Cosa abbiamo imparato** dalla fase appena chiusa: dati reali, limiti incontrati, numeri che cambiano le ipotesi iniziali.
2. **Le scelte da fare per il seguito**, ognuna con 2-3 alternative, pro e contro, e la tua raccomandazione motivata. Esempi: schema dei dati, definizione delle metriche, soglie statistiche, modello di esecuzione del conto demo.
3. **Il design di ciò che vedrò**: quali viste della dashboard servono per i dati appena prodotti, cosa mostra ognuna e come sono organizzate (descrizione o wireframe testuale). Le viste si accumulano checkpoint dopo checkpoint e confluiscono nella Fase 5.
4. **Domande aperte** per me.

Io scelgo. Ogni decisione presa diventa un **ADR** (Architecture Decision Record) in `docs/adr/NNNN-titolo.md`, nel formato MADR: contesto, opzioni considerate, decisione, conseguenze. Solo dopo che l'ADR è approvato e fuso in `main` pianifichi la fase successiva.

## Gestione del progetto su GitHub

Questo file vive nel repository. Imposta e usa questa struttura:

- **Milestone:** una per fase (`Fase 0 – Scheletro`, `Fase 1 – Ingestione`, …), più una per ogni checkpoint di design.
- **Issue:** dopo che approvo il piano di una fase, lo scomponi in issue piccole (idealmente completabili in una sessione), ognuna con descrizione, criterio di accettazione e milestone. Mostrami l'elenco prima di crearle.
- **Template delle issue** in `.github/ISSUE_TEMPLATE/`: `feature`, `bug`, `design-decision` (per i checkpoint e gli ADR), `research` (per verifiche come contratti, commissioni, fonti dati).
- **Label:** `fase:0`…`fase:5`; tipo (`type:feature`, `type:bug`, `type:design`, `type:research`, `type:chore`); area (`area:chain`, `area:ingest`, `area:analytics`, `area:signals`, `area:paper`, `area:infra`, `area:ui`); priorità (`prio:alta`, `prio:media`, `prio:bassa`).
- **GitHub Projects:** una board con colonne Backlog → Design → Pronto → In corso → Review → Fatto, alimentata dalle issue.
- **Branch e PR:** un branch per issue (`feat/123-nome-breve`, `fix/…`, `docs/…`), PR con `Closes #123` nella descrizione, merge squash. Template di PR in `.github/pull_request_template.md` con checklist: test, clippy, documentazione aggiornata, ADR collegato se la PR applica una decisione di design.
- **Protezione di `main`:** nessun push diretto, CI obbligatoria e verde per il merge.
- **Commit:** formato Conventional Commits (`feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `chore:`), così il changelog si genera in automatico.
- **Release:** un tag a fine di ogni fase (`v0.1.0` per la Fase 0, `v0.2.0` per la Fase 1, …) con `CHANGELOG.md` generato da git-cliff o release-please.
- **Manutenzione automatica:** Dependabot per le dipendenze Cargo e per le GitHub Actions; `cargo-deny` in CI per vulnerabilità note e licenze.
- **Documentazione nel repo:** `README.md` (avvio rapido), `CONTRACTS.md`, `docs/adr/`, `docs/design/`, `docs/metrics.md` (definizione precisa di ogni metrica e archetipo).
- **Nota per le sessioni cloud di Claude Code:** il proxy GitHub non permette le chiamate GraphQL, quindi `gh issue` e `gh pr` non funzionano. Usa gli strumenti GitHub integrati della sessione oppure le API REST con `gh api repos/{owner}/{repo}/…`.

## Stack tecnico

| Area | Scelta |
|---|---|
| Linguaggio | Rust stable, workspace Cargo multi-crate |
| Runtime async | tokio |
| Lettura chain | alloy 1.x (Polygon, WebSocket + HTTP, `sol!` per eventi e ABI) |
| Storico | backfill via RPC a blocchi (valuta cryo per l'estrazione in Parquet) |
| Dati relazionali | PostgreSQL con sqlx (mercati, wallet, posizioni, ledger del conto demo) |
| Serie temporali | QuestDB (trade, prezzi per esito, volumi) |
| Analisi pesante | DuckDB su Parquet per backtest e statistiche |
| Dashboard | Grafana; in seguito eventuale pannello web con axum |
| Alert | teloxide (Telegram), opzionale, solo notifiche |
| Infra locale | docker-compose con Postgres, QuestDB, Grafana |

## Architettura

```
crates/
  types/        # tipi condivisi: mercato, esito, trade, posizione, wallet
  config/       # caricamento config + validazione
  chain/        # client alloy, decodifica eventi, gestione reorg/finalità
  ingest/       # backfill storico + streaming live
  markets/      # mappatura token ID → mercato/esito, metadati e risoluzioni
  positions/    # ricostruzione posizioni e P&L per wallet
  analytics/    # metriche, classificazione wallet, anomalie
  signals/      # segnali (consenso a panieri, ecc.)
  paper/        # conto demo: portafoglio virtuale e modello di esecuzione
  backtest/     # rigioca lo storico usando lo stesso codice di signals e paper
  storage/      # sink Postgres e QuestDB
  datasource/   # trait MarketDataSource + adapter on-chain (+ adapter CLOB disabilitato)
bins/
  ingestor/     # servizio di ingestione
  analyzer/     # job di analisi periodici
  paper/        # servizio del conto demo
  backtest/     # CLI di backtest
infra/          # docker-compose, provisioning Grafana
```

Ogni crate si chiama con il prefisso del progetto (`sedette-types`, `sedette-chain`, `sedette-analytics`, `sedette-paper`, …) e così anche i binari (`sedette-ingestor`, `sedette-analyzer`, `sedette-demo`, `sedette-backtest`; il binario del conto demo si chiama `sedette-demo` per non entrare in conflitto con il crate `sedette-paper`). Il file di configurazione principale è `sedette.toml`, le variabili d'ambiente usano il prefisso `SEDETTE_`.

## Fasi

### Fase 0: Scheletro
- Workspace, trait principali, config, logging, docker-compose, CI GitHub Actions (fmt, clippy, test).
- `CONTRACTS.md` con gli indirizzi e gli eventi da usare, ognuno con la fonte ufficiale.
- Impostazione del repository secondo "Gestione del progetto su GitHub": milestone, label, template di issue e PR, board di Projects, Dependabot, `cargo-deny`, regole di protezione di `main` (quelle che richiedono permessi di amministratore elencamele come passaggi manuali).
- **Criterio di completamento:** l'infrastruttura parte con `docker compose up`, il binario `ingestor` si connette a Polygon e logga i nuovi blocchi, la struttura GitHub è pronta e la prima release `v0.1.0` è taggata.

### Fase 1: Ingestione on-chain 🎨
- Decodifica degli eventi necessari: esecuzione degli ordini sugli exchange (inclusa la variante neg-risk per i mercati multi-esito), registrazione dei token, split/merge/redeem delle posizioni, inizializzazione e risoluzione delle domande (il testo della domanda si ricava dai dati ancillari dell'oracolo).
- Mappatura token ID → mercato → esito, con titolo, categoria e stato (aperto/risolto, esito vincente).
- Backfill storico configurabile (per esempio ultimi 12 mesi) e streaming live.
- Gestione di reorg e finalità: Polygon ha riorganizzazioni non rare, quindi considera definitivi i dati solo dopo una profondità di blocchi configurabile.
- **Criterio di completamento:** su un campione di mercati risolti, volumi, prezzi ed esiti corrispondono a quelli visibili su un block explorer.
- **➜ Checkpoint di design** prima della Fase 2.

### Fase 2: Analisi dei wallet 🎨
- Ricostruzione di posizioni, costo medio e P&L realizzato e non realizzato per wallet.
- Metriche: rendimento per dollaro scambiato, numero di mercati risolti, P&L per categoria, drawdown massimo, distribuzione dei prezzi di ingresso, tempo medio di detenzione.
- **Classificazione degli archetipi** con regole esplicite e documentate:
  - *bot di spread / market maker* (volume enorme, rendimento per dollaro minimo, posizioni brevissime);
  - *bonder* (ingressi quasi sempre sopra 0,80-0,90 tenuti fino alla risoluzione);
  - *specialista* (concentrato in una categoria, rendimento per dollaro alto e stabile);
  - *balena* (poche operazioni enormi);
  - *rumore* (campione troppo piccolo per giudicare).
- **Fortuna o abilità:** test statistici (per esempio bootstrap sui risultati per mercato) con soglia minima di mercati risolti. Un wallet con meno di N mercati risolti non riceve un punteggio.
- **Anomalie e possibili insider:** wallet nuovi o dormienti che aprono posizioni grandi poco prima di movimenti forti o di una risoluzione, wallet che si muovono in modo coordinato. Etichettali come "anomalia", mai come accusa.
- **Criterio di completamento:** report che mostra la distribuzione del P&L su tutti i wallet, la classifica per archetipo e un elenco di anomalie, con le metriche spiegate in `docs/metrics.md`.
- **➜ Checkpoint di design** prima della Fase 3.

### Fase 3: Segnali
- **Consenso a panieri:** per ogni categoria, un paniere di specialisti selezionati **solo con dati antecedenti** alla finestra di test. Segnale quando una quota configurabile del paniere (per esempio 80%) si posiziona dallo stesso lato.
- Segnale da singolo specialista, con filtri su categoria e dimensione relativa della posizione.
- Ogni segnale registra: timestamp del blocco, mercato, esito, prezzo osservato, wallet coinvolti, motivazione.
- **Criterio di completamento:** storico dei segnali consultabile in Grafana.

### Fase 4: Conto demo (paper trading) 🎨
- Portafoglio virtuale con bankroll configurabile (per esempio 1.000 USDC virtuali), più strategie in parallelo, ognuna con il proprio sotto-conto.
- **Modello di esecuzione prudente.** Senza order book disponibile, il prezzo di ingresso simulato è il prezzo dei trade on-chain **successivi** al segnale, dopo un ritardo configurabile, più uno slippage proporzionale alla dimensione rispetto al volume recente, più le commissioni vigenti (verifica lo schema attuale delle commissioni e rendilo configurabile). Se dopo il segnale non ci sono trade abbastanza vicini nel tempo, l'operazione non viene eseguita.
- Gestione di uscite anticipate (regole configurabili) e incasso alla risoluzione (1 per l'esito vincente, 0 per il perdente).
- Regole di rischio: puntata massima per mercato, esposizione massima per categoria, perdita massima prima di sospendere la strategia.
- Ledger completo di ogni operazione simulata, curva del capitale e confronto con un benchmark ingenuo (per esempio comprare sempre il favorito).
- Il **backtest** usa esattamente lo stesso codice di `signals` e `paper` sullo storico, così i risultati demo e quelli del backtest sono confrontabili.
- **Criterio di completamento:** backtest su almeno 3 mesi di storico e almeno 4 settimane di conto demo live, con un report di rendimento, drawdown, numero di operazioni e intervalli di confidenza.
- **➜ Checkpoint di design** prima della Fase 5. Questo checkpoint è dedicato soprattutto all'interfaccia: raccogli le viste emerse nei checkpoint precedenti e proponi 2-3 impostazioni complete della dashboard (cosa c'è nella pagina principale, come si naviga, cosa va su Grafana e cosa eventualmente su un pannello web dedicato).

### Fase 5: Interfaccia
- Implementazione del design scelto nell'ultimo checkpoint. Come base, dashboard Grafana con: classifica dei wallet per archetipo, segnali recenti, anomalie, curva del capitale di ogni strategia demo.
- Bot Telegram opzionale, solo notifiche (nuovo segnale, anomalia, riepilogo giornaliero del conto demo). Accetta comandi solo da una whitelist di user ID.

## Cosa NON fare

- Non scrivere codice che piazza ordini, firma transazioni o gestisce chiavi private.
- Non chiamare domini di Polymarket e non aggiungere VPN, proxy o meccanismi per aggirare blocchi geografici.
- Non presentare un wallet come "insider" o colpevole: le anomalie sono segnalazioni statistiche.
- Non usare dati futuri nelle simulazioni e non selezionare i wallet guardando i risultati che dovrai poi testare.
- Non aggiungere dipendenze non necessarie. Motiva ogni crate esterno oltre a quelli in tabella.

## Primo passo

Leggi questo documento e rispondimi con:
1. eventuali dubbi o decisioni che spettano a me;
2. le fonti ufficiali che userai per contratti ed eventi;
3. il piano dettagliato della **Fase 0**, compresa la struttura GitHub (milestone, label, template);
4. l'elenco delle issue della Fase 0 che proponi di creare.

Non scrivere codice e non creare issue finché non approvo il piano.
