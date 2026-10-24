# Changelog

Tutte le modifiche rilevanti di Sedette. Una versione per fase: `v0.1.0` chiude la Fase 0, `v0.2.0` la Fase 1, e così via.
Il file è generato da [git-cliff](https://git-cliff.org/) a partire dai commit in formato
[Conventional Commits](https://www.conventionalcommits.org/it/v1.0.0/).

## [0.1.0] - 2026-10-24

### Funzionalità

- sedette-config con sedette.toml, variabili SEDETTE_ e telemetry ([#22](https://github.com/DanieleMarino70/Sedette/pull/22))
- tipi di base e trait MarketDataSource con adapter CLOB disabilitato ([#23](https://github.com/DanieleMarino70/Sedette/pull/23))
- sedette-chain e sedette-ingestor si connettono a Polygon e loggano i nuovi blocchi ([#24](https://github.com/DanieleMarino70/Sedette/pull/24))

### Documentazione

- README, CONTRIBUTING, specifica in docs/ e scheletri della documentazione ([#18](https://github.com/DanieleMarino70/Sedette/pull/18))
- ADR delle decisioni del Primo passo ([#26](https://github.com/DanieleMarino70/Sedette/pull/26))
- CONTRACTS.md con indirizzi, eventi, fonti e licenze verificati ([#27](https://github.com/DanieleMarino70/Sedette/pull/27))

### CI

- workflow CI, guardrail dei vincoli e controllo del titolo delle PR ([#20](https://github.com/DanieleMarino70/Sedette/pull/20))

### Manutenzione

- base del repo pubblico (.gitignore, licenze, SECURITY.md) ([#15](https://github.com/DanieleMarino70/Sedette/pull/15))
- struttura e sicurezza GitHub in script versionato ([#16](https://github.com/DanieleMarino70/Sedette/pull/16))
- template di issue (4 form) e di PR ([#17](https://github.com/DanieleMarino70/Sedette/pull/17))
- workspace Cargo e scheletro di tutti i crate e binari ([#19](https://github.com/DanieleMarino70/Sedette/pull/19))
- cargo-deny (avvisi, licenze, ban dei signer) e Dependabot ([#21](https://github.com/DanieleMarino70/Sedette/pull/21))
- infra docker compose (Postgres, QuestDB, Grafana) e provisioning dei datasource ([#25](https://github.com/DanieleMarino70/Sedette/pull/25))


