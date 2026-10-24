---
stato: accettato
data: 2026-10-23
decisori: Daniele Marino
issue: "#13"
---

# 0003. Fonti per contratti ed eventi, vincoli su Polymarket e fornitori di dati

## Contesto e problema

La specifica cita la "documentazione di Polymarket" tra le fonti ufficiali per indirizzi ed eventi, ma vieta anche
l'accesso ai domini di Polymarket. Servono regole chiare su quali fonti si consultano, sia durante la ricerca sia nel
funzionamento del sistema, e su eventuali fornitori di dati terzi.

La ricognizione del Primo passo ha mostrato che i contratti sono cambiati più volte: tre generazioni di exchange (v1, V2 dal
2026-04-28, Protocol V2 con Exchange V3) e il collaterale pUSD.

## Fattori decisionali

- Vincolo legale della specifica: nessun accesso a polymarket.com e sottodomini.
- Verificabilità: ogni dato deve avere una fonte citabile.
- Termini d'uso dei fornitori terzi.

## Opzioni considerate

1. Niente polymarket.com, nemmeno in lettura per la ricerca: solo GitHub, Polygonscan e fonti on-chain.
2. Consultare docs.polymarket.com in sola lettura durante la ricerca, mai dal codice.

## Decisione

Opzione scelta: **1**. Gerarchia delle fonti per `CONTRACTS.md`:

1. codice Solidity nei repository ufficiali (`github.com/Polymarket`, `gnosis`, `UMAprotocol`), fissato a un commit;
2. contratti verificati su Polygonscan e log reali on-chain;
3. configurazioni degli SDK e dei subgraph ufficiali, lette come file su GitHub senza chiamarne le API;
4. fonti terze solo come conferma di date, segnalate come tali.

Ciò che esiste solo su polymarket.com resta marcato "non verificato".

Fornitori di dati terzi (Goldsky, The Graph, Dune, Bitquery, Allium): **nessuno** in Fase 0 e Fase 1. Riassunto dei termini
valutati: Goldsky solo per uso interno e subgraph pubblici di Polymarket non più affidabili; The Graph per uso personale non
commerciale e senza bot; Dune con API a pagamento e senza scraper; Bitquery con licenza commerciale nel piano Pro; Allium in
beta. Potranno servire solo come verifica incrociata, dopo una decisione esplicita.

Il provider RPC (D6) si sceglie nel piano della Fase 1, in base ai limiti di `eth_getLogs`. In Fase 0 basta un endpoint
WebSocket qualsiasi, configurato solo in `.env`.

### Conseguenze

- Positive: rispetto pieno del vincolo legale; ogni dato ha una fonte verificabile on-chain o nel codice.
- Negative: alcune informazioni (per esempio la tabella ufficiale degli indirizzi e lo schema delle commissioni per
  categoria) restano "non verificate"; i parametri di commissione per mercato non sono disponibili e le commissioni si
  ricostruiranno dagli eventi on-chain (`OrderFilled.fee`, `FeeCharged`, `FeeRefunded`).
- Licenze delle fonti: i repository `ctf-exchange-v2` e `polymarket-v2-external` sono sotto BUSL-1.1 (verificato durante
  la issue #5). In Fase 1 le firme degli eventi si dichiarano nel codice di Sedette a partire dall'ABI pubblica, senza
  copiare file sorgente di quei repository; la scelta va confermata nel piano della Fase 1.

## Pro e contro delle opzioni

### Opzione 1 – Solo GitHub, Polygonscan e on-chain

- Pro: nessuna ambiguità legale; fonti verificabili.
- Contro: più lavoro di verifica; qualche dato resta aperto.

### Opzione 2 – docs.polymarket.com in sola lettura

- Pro: accesso alla documentazione più completa.
- Contro: contraddice il divieto sui domini di Polymarket; possibile blocco geografico da non aggirare.

## Riferimenti

- Piano del Primo passo, decisioni D3, D6 e D7; fatti F1–F4.
- Issue #5 (`CONTRACTS.md`).
