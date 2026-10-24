//! `sedette-ingestor`: servizio di ingestione dei dati on-chain.
//!
//! Fase 0: si connette a Polygon via WebSocket, registra nei log ogni nuovo
//! blocco e, periodicamente, il ritardo del blocco `finalized`. Si riconnette
//! con backoff esponenziale e si arresta in modo pulito con Ctrl+C (o SIGTERM).
//!
//! Uso: `sedette-ingestor [--config <file>]`. Senza `--config` legge
//! `sedette.toml` dalla directory corrente, se esiste. I segreti arrivano da
//! variabili d'ambiente o dal file `.env` locale (vedi `.env.example`).

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::Duration;

use anyhow::{Context, bail};
use futures_util::StreamExt;
use sedette_chain::{Backoff, ChainClient, ChainError, HeadEvent, HeadTracker};
use sedette_config::{DEFAULT_CONFIG_FILE, SecretUrl, Settings, telemetry};
use tracing::{debug, error, info, warn};

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            // Il logging potrebbe non essere ancora attivo: si scrive anche su stderr.
            error!("{err:#}");
            eprintln!("sedette-ingestor: {err:#}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> anyhow::Result<()> {
    let config_path = config_path(std::env::args().skip(1))?;
    let env = environment(Path::new(".env"))?;
    let settings = Settings::load(config_path.as_deref(), env)?;
    telemetry::init(&settings.logging)?;

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .context("avvio del runtime tokio")?;
    runtime.block_on(ingest(settings))
}

/// File di configurazione: `--config <file>`, altrimenti `sedette.toml` se esiste.
fn config_path(mut args: impl Iterator<Item = String>) -> anyhow::Result<Option<PathBuf>> {
    match args.next().as_deref() {
        None => {
            let default = Path::new(DEFAULT_CONFIG_FILE);
            Ok(default.exists().then(|| default.to_path_buf()))
        }
        Some("--config") => match args.next() {
            Some(path) => Ok(Some(PathBuf::from(path))),
            None => bail!("--config richiede il percorso di un file"),
        },
        Some(other) => {
            bail!("argomento sconosciuto: {other} (uso: sedette-ingestor [--config <file>])")
        }
    }
}

/// Variabili d'ambiente del processo, completate con quelle del file `.env` se
/// esiste. L'ambiente del processo ha la precedenza e non viene modificato.
fn environment(dotenv: &Path) -> anyhow::Result<HashMap<String, String>> {
    let mut vars = HashMap::new();
    if dotenv.exists() {
        for item in dotenvy::from_path_iter(dotenv).context("lettura di .env")? {
            let (key, value) = item.context("riga non valida in .env")?;
            vars.insert(key, value);
        }
    }
    vars.extend(
        std::env::vars_os()
            .filter_map(|(k, v)| Some((k.into_string().ok()?, v.into_string().ok()?))),
    );
    Ok(vars)
}

/// Ciclo principale: sessioni successive con riconnessione, fino all'arresto.
async fn ingest(settings: Settings) -> anyhow::Result<()> {
    let endpoint = settings.require_ws_url()?.clone();
    let mut backoff = Backoff::new(
        Duration::from_millis(settings.ingestor.reconnect_initial_backoff_ms),
        Duration::from_secs(settings.ingestor.reconnect_max_backoff_secs),
    );
    let mut tracker = HeadTracker::new();

    info!(
        endpoint = %endpoint,
        chain_id = settings.chain.expected_chain_id,
        "avvio di sedette-ingestor"
    );

    let shutdown = shutdown_signal();
    tokio::pin!(shutdown);

    loop {
        tokio::select! {
            () = &mut shutdown => break,
            result = session(&endpoint, &settings, &mut tracker, &mut backoff) => match result {
                Ok(()) => warn!("la sottoscrizione ai nuovi blocchi si è chiusa"),
                Err(err) if err.is_fatal() => return Err(err.into()),
                Err(err) => warn!(error = %err, "sessione interrotta"),
            },
        }

        let delay = backoff.next_delay();
        info!(
            attesa_ms = delay.as_millis() as u64,
            "nuovo tentativo di connessione"
        );
        tokio::select! {
            () = &mut shutdown => break,
            () = tokio::time::sleep(delay) => {}
        }
    }

    info!("arresto richiesto: sedette-ingestor termina");
    Ok(())
}

/// Una connessione: sottoscrizione ai nuovi blocchi e lettura periodica del
/// blocco `finalized`. Termina quando il flusso si chiude o con un errore.
async fn session(
    endpoint: &SecretUrl,
    settings: &Settings,
    tracker: &mut HeadTracker,
    backoff: &mut Backoff,
) -> Result<(), ChainError> {
    let client = ChainClient::connect_ws(endpoint, settings.chain.expected_chain_id).await?;
    let mut heads = client.subscribe_heads().await?;
    backoff.reset();
    info!(endpoint = %client.endpoint(), "connesso a Polygon, in ascolto dei nuovi blocchi");

    let mut poll =
        tokio::time::interval(Duration::from_secs(settings.ingestor.finalized_poll_secs));
    poll.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

    loop {
        tokio::select! {
            head = heads.next() => {
                let Some(head) = head else { return Ok(()) };
                match tracker.observe(head) {
                    HeadEvent::First | HeadEvent::Next => info!(
                        numero = head.number,
                        hash = %head.hash,
                        timestamp = head.timestamp,
                        "nuovo blocco"
                    ),
                    HeadEvent::Gap { missing } => warn!(
                        numero = head.number,
                        hash = %head.hash,
                        mancanti = missing,
                        "nuovo blocco dopo un buco: blocchi intermedi non ricevuti"
                    ),
                    HeadEvent::Reorg { depth } => warn!(
                        numero = head.number,
                        hash = %head.hash,
                        padre = %head.parent_hash,
                        profondita = depth,
                        "possibile riorganizzazione: il blocco non discende dal precedente"
                    ),
                }
            }
            _ = poll.tick() => match client.finalized().await {
                Ok(Some(finalized)) => {
                    let latest = tracker.last().map_or(finalized.number, |b| b.number);
                    info!(
                        finalized = finalized.number,
                        ritardo_blocchi = latest.saturating_sub(finalized.number),
                        "blocco finalized"
                    );
                }
                Ok(None) => warn!("il nodo non fornisce il blocco finalized"),
                Err(err) => debug!(error = %err, "lettura del blocco finalized non riuscita"),
            },
        }
    }
}

/// Si completa con Ctrl+C e, su Unix, anche con SIGTERM (per esempio `docker stop`).
async fn shutdown_signal() {
    let ctrl_c = async {
        if let Err(err) = tokio::signal::ctrl_c().await {
            warn!(error = %err, "impossibile ascoltare Ctrl+C");
            std::future::pending::<()>().await;
        }
    };

    #[cfg(unix)]
    let terminate = async {
        match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
            Ok(mut signal) => {
                signal.recv().await;
            }
            Err(err) => {
                warn!(error = %err, "impossibile ascoltare SIGTERM");
                std::future::pending::<()>().await;
            }
        }
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {}
        () = terminate => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(list: &[&str]) -> impl Iterator<Item = String> {
        list.iter()
            .map(|s| (*s).to_owned())
            .collect::<Vec<_>>()
            .into_iter()
    }

    #[test]
    fn argomenti() {
        assert_eq!(
            config_path(args(&["--config", "altro.toml"])).unwrap(),
            Some(PathBuf::from("altro.toml"))
        );
        assert!(config_path(args(&["--config"])).is_err());
        assert!(config_path(args(&["--boh"])).is_err());
    }

    #[test]
    fn dotenv_assente_non_e_un_errore() {
        let vars = environment(Path::new("questo-file-non-esiste.env")).unwrap();
        assert!(!vars.is_empty());
    }
}
