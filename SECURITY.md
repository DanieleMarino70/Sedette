# Sicurezza

## Perimetro del progetto

Sedette è un motore di analisi in **sola lettura** di dati pubblici della blockchain Polygon. Per costruzione:

- **non piazza ordini**, non firma transazioni e non contiene codice per farlo;
- **non gestisce chiavi private** e non si collega a nessun wallet;
- **non accede ai siti o alle API di Polymarket** (dominio `polymarket.com` e sottodomini) e non usa VPN, proxy o altre tecniche per aggirare blocchi geografici;
- il conto demo è una simulazione con un bankroll virtuale.

La CI fa rispettare questi vincoli: rifiuta riferimenti ai domini di Polymarket nel codice e feature di firma nelle dipendenze. Una modifica che li violi è considerata un problema di sicurezza.

## Segreti

- I segreti (URL RPC con chiave, password dei database e di Grafana) si passano **solo come variabili d'ambiente**, in un file `.env` locale che non va mai versionato. `.env.example` contiene solo segnaposto.
- Sul repository sono attivi il secret scanning e la push protection di GitHub.
- La CI non usa segreti.

Se hai pubblicato per errore una chiave in un commit, una issue o una PR, **considerala compromessa**: revocala subito dal pannello del provider e generane una nuova. Rimuoverla dalla storia di git non basta.

## Come segnalare una vulnerabilità

Non aprire una issue pubblica. Usa la segnalazione privata di GitHub:
[Security → Report a vulnerability](https://github.com/DanieleMarino70/Sedette/security/advisories/new).

Descrivi il problema, come riprodurlo e l'impatto che ti aspetti. Riceverai una risposta appena possibile; il progetto è mantenuto da una sola persona e non offre SLA.

## Versioni supportate

Il progetto è in sviluppo attivo. Le correzioni di sicurezza vengono applicate solo a `main` e all'ultima release.

## Politica dei dati

Il repository è pubblico, quindi:

- **nessun dato grezzo o derivato** e nessun export (Parquet, DuckDB, dump dei database, CSV) va versionato. I dati vivono nei volumi Docker locali e nelle cartelle escluse da `.gitignore`;
- **nessun elenco di wallet** classificati, in classifica o marcati come "anomalia" compare nel repository, nemmeno nei documenti di checkpoint in `docs/design/` o nei report. Nel repo vanno solo aggregati, distribuzioni e metodologia;
- i risultati a livello di singolo wallet restano nei database locali e su Grafana, raggiungibile solo da `127.0.0.1`;
- negli esempi e nei test si usano indirizzi sintetici, non indirizzi reali;
- un'anomalia è una segnalazione statistica, **mai un'accusa**: nessun documento del progetto presenta un wallet come "insider" o colpevole.
