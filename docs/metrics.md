# Metriche e archetipi

Questo documento definisce con precisione ogni metrica e ogni archetipo usati da Sedette. Le definizioni si scrivono nella
fase che le introduce (soprattutto la Fase 2) e cambiano solo tramite un ADR.

Per ogni metrica il documento riporterà:

- **definizione** formale (formula, unità, finestra temporale);
- **dati di partenza** (eventi on-chain usati) e trattamento dei casi limite;
- **soglia minima di campione** sotto la quale la metrica non viene calcolata o non viene mostrata;
- **incertezza**: come si stima (per esempio intervallo bootstrap) e come si riporta.

## Principi

- **Niente look-ahead bias.** Ogni valore usato in una decisione simulata si calcola solo con dati disponibili in quel
  momento (timestamp del blocco).
- **Niente survivorship bias.** I wallet "forti" si selezionano con dati passati, non con il risultato finale che si vuole
  testare.
- **Campione e incertezza dichiarati.** Ogni numero mostrato è accompagnato dalla dimensione del campione e da una misura di
  incertezza.
- **Anomalie, non accuse.** Le anomalie sono segnalazioni statistiche.

## Indice

### Posizioni e P&L (Fase 2)

- Costo medio di una posizione — *da definire*
- P&L realizzato — *da definire*
- P&L non realizzato — *da definire*

### Metriche per wallet (Fase 2)

- Rendimento per dollaro scambiato — *da definire*
- Numero di mercati risolti — *da definire*
- P&L per categoria — *da definire*
- Drawdown massimo — *da definire*
- Distribuzione dei prezzi di ingresso — *da definire*
- Tempo medio di detenzione — *da definire*

### Archetipi (Fase 2)

Regole esplicite e documentate, applicate nell'ordine in cui verranno definite:

- Bot di spread / market maker — *da definire*
- Bonder — *da definire*
- Specialista — *da definire*
- Balena — *da definire*
- Rumore (campione troppo piccolo per giudicare) — *da definire*

### Fortuna o abilità (Fase 2)

- Test statistico (per esempio bootstrap sui risultati per mercato) — *da definire*
- Soglia minima N di mercati risolti per ricevere un punteggio — *da definire*

### Anomalie (Fase 2)

- Wallet nuovi o dormienti con posizioni grandi poco prima di movimenti forti o di una risoluzione — *da definire*
- Wallet che si muovono in modo coordinato — *da definire*

### Segnali (Fase 3)

- Consenso a panieri (quota del paniere dallo stesso lato) — *da definire*
- Segnale da singolo specialista — *da definire*

### Conto demo e backtest (Fase 4)

- Prezzo di ingresso simulato (ritardo, slippage, commissioni) — *da definire*
- Rendimento, drawdown, numero di operazioni e intervalli di confidenza — *da definire*
- Benchmark ingenuo (per esempio comprare sempre il favorito) — *da definire*
