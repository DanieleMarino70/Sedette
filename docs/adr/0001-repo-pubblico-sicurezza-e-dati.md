---
stato: accettato
data: 2026-10-23
decisori: Daniele Marino
issue: "#13"
---

# 0001. Repository pubblico, sistema di sicurezza e politica dei dati

## Contesto e problema

La specifica chiede che `main` sia protetto (nessun push diretto, CI obbligatoria). Il repository era privato su un account
GitHub Free, dove la protezione dei branch e i ruleset non sono disponibili: le API rispondevano 403 "Upgrade to GitHub Pro
or make this repository public". Sui repository privati Free anche i campi obbligatori dei form delle issue non funzionano
e le Actions hanno 2.000 minuti al mese.

Rendere pubblico il repository apre però due rischi: la pubblicazione accidentale di segreti (URL RPC con chiave) e la
pubblicazione di risultati a livello di wallet che potrebbero sembrare accuse.

## Fattori decisionali

- Vincolo della specifica: protezione di `main` e CI obbligatoria.
- Nessun segreto deve mai finire nel repository.
- Un'anomalia è una segnalazione statistica, mai un'accusa.
- Costo.

## Opzioni considerate

1. Repository pubblico con un sistema di sicurezza.
2. GitHub Pro (circa 4 $ al mese), repository privato.
3. Repository privato senza protezione ("honor system").

## Decisione

Opzione scelta: **1, repository pubblico con un sistema di sicurezza**, licenza **MIT OR Apache-2.0**. Le impostazioni da
amministratore le applica Claude via API (D11), versionate in `scripts/github/bootstrap.sh`.

Sistema di sicurezza:

- secret scanning e **push protection**; Dependabot alerts e security updates; segnalazione privata delle vulnerabilità;
  CodeQL in default setup (Rust e GitHub Actions);
- ruleset su `main`: PR obbligatoria con merge squash, storia lineare, niente force push né cancellazione, nessun bypass,
  check obbligatori (`fmt`, `clippy`, `test`, `deny`, `guardrail`, `compose`, `pr-title`) e risultati di CodeQL;
- ruleset sui tag `v*`: niente cancellazione né spostamento;
- Actions: `GITHUB_TOKEN` in sola lettura, nessun segreto in CI, azioni fissate per commit SHA, approvazione per le PR da
  fork esterni;
- `scripts/ci/guardrail.sh` in CI: niente domini di Polymarket nel codice, niente signer di alloy, niente `.env` o URL RPC
  con chiave versionati;
- `.gitignore` per `.env`, dati ed export; `SecretUrl` maschera gli URL RPC nei log e negli errori.

Politica dei dati (in `SECURITY.md`): nel repository niente dati grezzi o derivati, niente export, niente elenchi di wallet
classificati, in classifica o marcati come anomalia. Documenti e report contengono solo aggregati e metodologia; i
risultati per wallet restano nei database locali e su Grafana, esposto solo su `127.0.0.1`.

### Conseguenze

- Positive: protezione di `main` applicata dalla piattaforma; strumenti di sicurezza e Actions gratuiti; campi obbligatori
  nei form.
- Negative: il codice, la specifica e la cronologia delle issue sono visibili a tutti; un errore di pubblicazione è di fatto
  irreversibile (il contenuto può essere copiato o indicizzato).
- Da verificare: la push protection blocca davvero un segreto noto (verifica descritta, non eseguita con segreti reali); il
  guardrail resta allineato ai formati degli URL dei provider.

## Pro e contro delle opzioni

### Opzione 1 – Pubblico

- Pro: protezione e ruleset disponibili; CodeQL e Actions gratuiti; nessun costo.
- Contro: esposizione pubblica; richiede una politica dei dati rigorosa.

### Opzione 2 – GitHub Pro

- Pro: repository privato con protezione.
- Contro: costo ricorrente; CodeQL non disponibile sui privati senza GitHub Advanced Security; 3.000 minuti di Actions.

### Opzione 3 – Privato senza protezione

- Pro: nessun costo, nessuna esposizione.
- Contro: la regola "nessun push diretto, CI verde" resta solo una promessa; contraddice la specifica.

## Riferimenti

- Piano del Primo passo, decisioni D1 e D11.
- [SECURITY.md](../../SECURITY.md), [CONTRIBUTING.md](../../CONTRIBUTING.md#struttura-github),
  [scripts/github/bootstrap.sh](../../scripts/github/bootstrap.sh), [scripts/ci/guardrail.sh](../../scripts/ci/guardrail.sh).
- PR #15, #16, #20, #21.
