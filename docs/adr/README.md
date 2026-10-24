# Architecture Decision Record

Ogni decisione di architettura o di metodo presa nel progetto è registrata qui, in un file per decisione, nel formato
[MADR](https://adr.github.io/madr/): contesto, opzioni considerate, decisione, conseguenze.

## Convenzioni

- Nome del file: `NNNN-titolo-breve.md`, con `NNNN` progressivo a quattro cifre (`0001-…`, `0002-…`).
- Si parte da [0000-template.md](0000-template.md).
- Stati possibili: `proposto`, `accettato`, `rifiutato`, `deprecato`, `sostituito da ADR-NNNN`.
- Un ADR accettato non si riscrive: se la decisione cambia, se ne scrive uno nuovo che lo sostituisce, e il vecchio passa a
  `sostituito da`.
- Ogni ADR nasce da una issue `type:design` e viene fuso in `main` con una PR. La fase successiva si pianifica solo dopo il
  merge.

## Indice

| ADR | Titolo | Stato |
|---|---|---|
| [0001](0001-repo-pubblico-sicurezza-e-dati.md) | Repository pubblico, sistema di sicurezza e politica dei dati | accettato |
| [0002](0002-alloy-2.md) | alloy 2.x invece della 1.x indicata nella specifica | accettato |
| [0003](0003-fonti-e-vincoli-polymarket.md) | Fonti per contratti ed eventi, vincoli su Polymarket e fornitori di dati | accettato |
| [0004](0004-scelte-di-stack-minori.md) | Scelte di stack minori della Fase 0 | accettato |
| [0005](0005-processo-github-e-release.md) | Processo di lavoro su GitHub e release | accettato |
| [0006](0006-finalita-su-polygon.md) | Quando considerare definitivo un blocco su Polygon PoS | proposto |
