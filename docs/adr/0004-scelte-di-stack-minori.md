---
stato: accettato
data: 2026-10-23
decisori: Daniele Marino
issue: "#13"
---

# 0004. Scelte di stack minori della Fase 0

## Contesto e problema

La Fase 0 richiede alcune scelte di dettaglio non fissate dalla specifica. La specifica chiede di motivare ogni crate esterno
oltre a quelli previsti.

## Decisioni

| Tema | Scelta | Alternative scartate | Motivo |
|---|---|---|---|
| Configurazione | crate `config` (config-rs), solo feature `toml` | figment; parsing manuale | Unisce TOML e variabili `SEDETTE_` con chiavi annidate; mantenuto (release 2026-09). figment non ha release dal 2024. |
| File `.env` | `dotenvy`, solo nei binari, letto come iteratore | `dotenvy::dotenv()`; nessun supporto | Non chiama `set_var` (in edition 2024 è `unsafe`): le variabili si passano esplicitamente a `Settings::load`. |
| Changelog | git-cliff, tag manuali per fase | release-please | release-please non aggiorna i crate con `version.workspace = true` (issue #2111) e vuole gestire le versioni con PR proprie. |
| Struttura | scheletro di tutti i crate e binari già in Fase 0 | crate creati quando servono | I nomi e le dipendenze tra crate sono fissati subito, come nell'architettura della specifica. |
| Binario di backtest | package `sedette-backtest-cli`, binario `sedette-backtest` | rinominare il crate | La specifica usa `sedette-backtest` sia per il crate sia per il binario; i package di un workspace devono avere nomi diversi. Il nome del binario resta quello della specifica. |
| Azioni GitHub | fissate per commit SHA, versione in commento | tag mobili (`@v2`) | Un tag può essere spostato; Dependabot aggiorna gli SHA. |
| Tipi primitivi | `alloy-primitives` 1.7 in `sedette-types` | dipendere da `alloy` | Tipi `B256`, `U256`, `Address` senza trascinare provider e trasporti nel crate dei tipi; compatibile con alloy 2.5. |
| Stream | `futures-util` | — | `StreamExt` per consumare la sottoscrizione, come negli esempi ufficiali di alloy. |
| Log | `tracing-subscriber` (`env-filter`, `json`) | — | Necessario per far uscire i log di `tracing`. |
| cargo-deny | grafo limitato alle piattaforme di build; eccezione RUSTSEC-2026-0387 | aggiungere `Unlicense`; nessuna eccezione | Le dipendenze solo wasm32 di `alloy-transport-ws` non entrano nei binari. `crypto-bigint 0.5.5` arriva solo dal trait `alloy-signer`: Sedette non elabora chiavi né firme; l'eccezione si rimuove quando alloy si aggiorna. |
| Test in locale | CI su Linux come riferimento; container `rust:1` in locale | disattivare lo Smart App Control di Windows | Sul PC di sviluppo lo Smart App Control blocca a intermittenza gli eseguibili di test non firmati (os error 4551); è un'impostazione di sicurezza dell'utente, da non toccare. |

### Conseguenze

- Positive: dipendenze minime e motivate; configurazione testabile senza stato globale; supply chain delle Actions protetta.
- Negative: un package si chiama `sedette-backtest-cli` (`cargo run -p sedette-backtest-cli`); l'eccezione di cargo-deny va
  ricontrollata a ogni aggiornamento di alloy.

## Riferimenti

- Piano del Primo passo, decisione D10.
- PR #19, #21, #22, #23, #24.
