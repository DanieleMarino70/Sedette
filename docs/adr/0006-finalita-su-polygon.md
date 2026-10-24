---
stato: proposto
data: 2026-10-24
decisori: Daniele Marino
issue: "#13"
---

# 0006. Quando considerare definitivo un blocco su Polygon PoS

## Contesto e problema

La specifica chiede di considerare definitivi i dati solo dopo una profondità di blocchi configurabile, perché "Polygon ha
riorganizzazioni non rare". Le fonti ufficiali verificate nel Primo passo descrivono una situazione cambiata:

- Heimdall v2 (luglio 2025): finalità deterministica tramite "milestone" in 2–5 secondi; esiste il tag RPC `finalized`.
- Rio (ottobre 2025): i reorg sono descritti come "essenzialmente eliminati".
- La finalità può però bloccarsi: il 2025-09-10 è rimasta ferma 10–15 minuti.

Osservazione in Fase 0 (`sedette-ingestor` contro un nodo pubblico, 2026-10-24): il blocco `finalized` resta 0–2 blocchi
dietro la testa; un blocco ogni 1,5 secondi circa.

## Opzioni considerate

1. Solo profondità fissa (N conferme), come scritto nella specifica.
2. Solo tag `finalized`.
3. Definitivo se si trova sotto il tag `finalized` **oppure** se ha almeno N conferme; allarme se il ritardo del
   `finalized` cresce oltre una soglia.

## Decisione proposta

Opzione **3**, con N configurabile (`finality.confirmations`, oggi 32) e `finality.use_finalized_tag = true`. Decisione da
confermare nel piano della Fase 1, dopo aver misurato su più giorni il ritardo del `finalized` e la frequenza dei reorg
osservati.

### Conseguenze

- Positive: latenza bassa nel caso normale; resta un limite di sicurezza se la finalità si blocca.
- Negative: due parametri da tarare; serve monitorare il ritardo del `finalized`.

## Riferimenti

- Piano del Primo passo, fatto F6.
- <https://docs.polygon.technology/pos/concepts/finality/finality/>
- <https://polygon.technology/blog/polygon-5-second-fast-finality-upgrade>
- <https://status.polygon.technology/incidents/c8nc05x3qvmg>
- PR #24 (log del ritardo del `finalized`).
