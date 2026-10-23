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
| — | Nessun ADR ancora registrato | — |
