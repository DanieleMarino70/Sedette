# Checkpoint di design

Dopo le fasi importanti (Fase 1, Fase 2, Fase 4) ci si ferma per un checkpoint di design prima di proseguire.
Ogni checkpoint produce un documento in questa cartella e, per ogni scelta fatta, un ADR in [../adr/](../adr/README.md).

| Checkpoint | Dopo | Milestone |
|---|---|---|
| 1 | Fase 1 – Ingestione | Checkpoint 1 – Design post-ingestione |
| 2 | Fase 2 – Analisi wallet | Checkpoint 2 – Design post-analisi |
| 3 | Fase 4 – Conto demo | Checkpoint 3 – Design interfaccia |

## Struttura del documento

Nome del file: `checkpoint-N-titolo-breve.md`. Il documento contiene quattro sezioni.

1. **Cosa abbiamo imparato** dalla fase appena chiusa: dati reali, limiti incontrati, numeri che cambiano le ipotesi iniziali.
2. **Le scelte da fare per il seguito**, ognuna con 2-3 alternative, pro e contro, e una raccomandazione motivata (per
   esempio: schema dei dati, definizione delle metriche, soglie statistiche, modello di esecuzione del conto demo).
3. **Il design di ciò che si vedrà**: quali viste della dashboard servono per i dati appena prodotti, cosa mostra ognuna e
   come sono organizzate (descrizione o wireframe testuale). Le viste si accumulano di checkpoint in checkpoint e
   confluiscono nella Fase 5. Il checkpoint 3 è dedicato soprattutto all'interfaccia: propone 2-3 impostazioni complete
   della dashboard.
4. **Domande aperte.**

## Regole sui dati

Il repository è pubblico. I documenti di checkpoint contengono solo **aggregati, distribuzioni e metodologia**: niente
indirizzi di wallet classificati, in classifica o marcati come anomalia. Gli esempi usano indirizzi sintetici. Vedi la
politica dei dati in [SECURITY.md](../../SECURITY.md#politica-dei-dati).
