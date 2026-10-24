# Sedette

Motore di analisi dei mercati predittivi di Polymarket basato **esclusivamente su dati pubblici on-chain** (Polygon PoS),
con un **conto demo** (paper trading) che simula strategie con un bankroll virtuale.

> **Stato:** Fase 0 (scheletro) in corso. L'avanzamento è sulla [board](https://github.com/users/DanieleMarino70/projects/5)
> e nelle [milestone](https://github.com/DanieleMarino70/Sedette/milestones).

## Obiettivi

1. Capire quali wallet hanno un vantaggio reale e misurabile, separandoli da fortuna, bot di spread e possibili insider.
2. Verificare con numeri onesti se una strategia (per esempio copiare panieri di specialisti) rende **dopo** ritardi,
   slippage e commissioni.

## Cosa non fa

Sedette è in **sola lettura**:

- non piazza ordini, non firma transazioni e non gestisce chiavi private;
- non accede ai siti o alle API di Polymarket e non usa VPN o proxy per aggirare blocchi geografici: i dati arrivano dalla
  blockchain pubblica tramite un provider RPC di Polygon;
- un'anomalia è una segnalazione statistica, mai un'accusa.

I dettagli sono in [SECURITY.md](SECURITY.md).

## Avvio rapido

Prerequisiti:

- [Rust](https://rustup.rs/) stable (la versione minima è indicata in `Cargo.toml`);
- Docker con `docker compose` (su Windows, Docker Desktop con backend WSL2);
- un endpoint RPC **WebSocket** di Polygon PoS mainnet (chain id 137), per esempio da un provider con piano gratuito.

```bash
git clone https://github.com/DanieleMarino70/Sedette.git
cd Sedette

# 1. Segreti: copia il modello e inserisci l'URL RPC e le password locali.
#    .env non va mai versionato.
cp .env.example .env

# 2. Infrastruttura locale: Postgres, QuestDB e Grafana, esposti solo su 127.0.0.1.
docker compose up -d
docker compose ps

# 3. Ingestor: si connette a Polygon e registra i nuovi blocchi.
cargo run -p sedette-ingestor
```

| Servizio | Indirizzo |
|---|---|
| Grafana | <http://127.0.0.1:3000> |
| Console QuestDB | <http://127.0.0.1:9000> |
| Postgres | `127.0.0.1:5432` |

## Struttura del repository

```
crates/
  types/        tipi condivisi: mercato, esito, trade, posizione, wallet
  config/       caricamento della configurazione e validazione
  chain/        client alloy, decodifica degli eventi, reorg e finalità
  ingest/       backfill storico e streaming live
  markets/      mappatura token ID → mercato/esito, metadati e risoluzioni
  positions/    ricostruzione di posizioni e P&L per wallet
  analytics/    metriche, classificazione dei wallet, anomalie
  signals/      segnali (consenso a panieri, ecc.)
  paper/        conto demo: portafoglio virtuale e modello di esecuzione
  backtest/     rigioca lo storico con lo stesso codice di signals e paper
  storage/      sink Postgres e QuestDB
  datasource/   trait MarketDataSource e adapter
bins/
  ingestor/     sedette-ingestor: servizio di ingestione
  analyzer/     sedette-analyzer: job di analisi periodici
  paper/        sedette-demo: servizio del conto demo
  backtest/     sedette-backtest: CLI di backtest
infra/          docker compose e provisioning di Grafana
scripts/        script di manutenzione (configurazione GitHub)
docs/           specifica, ADR, checkpoint di design, metriche
```

## Documentazione

| Documento | Contenuto |
|---|---|
| [docs/PROMPT.md](docs/PROMPT.md) | Specifica del progetto: vincoli, fasi, criteri di completamento. |
| [CONTRACTS.md](CONTRACTS.md) | Indirizzi, eventi e fonti ufficiali dei contratti on-chain. |
| [docs/adr/](docs/adr/README.md) | Architecture Decision Record. |
| [docs/design/](docs/design/README.md) | Documenti dei checkpoint di design. |
| [docs/metrics.md](docs/metrics.md) | Definizione di metriche e archetipi. |
| [CONTRIBUTING.md](CONTRIBUTING.md) | Flusso di lavoro, convenzioni, struttura GitHub. |
| [SECURITY.md](SECURITY.md) | Perimetro, segreti, segnalazioni, politica dei dati. |

## Licenza

Distribuito a scelta sotto una delle due licenze:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

Salvo indicazione contraria, ogni contributo inviato intenzionalmente per l'inclusione nel progetto, come definito nella
licenza Apache-2.0, è distribuito con la stessa doppia licenza, senza termini o condizioni aggiuntivi.
