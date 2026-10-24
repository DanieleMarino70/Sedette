//! Inizializzazione dei log con `tracing`, secondo la sezione `[logging]`.

use std::io::IsTerminal;

use tracing::Subscriber;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer, fmt};

use crate::settings::{LogFormat, LoggingSettings};

/// Errori di inizializzazione dei log.
#[derive(Debug, thiserror::Error)]
pub enum TelemetryError {
    /// `logging.filter` non è una direttiva valida per `EnvFilter`.
    #[error("logging.filter non valido: {0}")]
    Filter(#[from] tracing_subscriber::filter::ParseError),
    /// Un subscriber globale era già stato installato.
    #[error("logging già inizializzato: {0}")]
    AlreadyInitialized(#[from] tracing_subscriber::util::TryInitError),
}

/// Installa il subscriber globale: filtro da `logging.filter`, formato da
/// `logging.format`, uscita su stdout. I colori ANSI si usano solo se stdout
/// è un terminale.
pub fn init(settings: &LoggingSettings) -> Result<(), TelemetryError> {
    let filter = filter(settings)?;
    let ansi = std::io::stdout().is_terminal();
    tracing_subscriber::registry()
        .with(filter)
        .with(fmt_layer(settings.format, ansi, std::io::stdout))
        .try_init()?;
    Ok(())
}

/// Filtro dei log costruito da `logging.filter`.
pub fn filter(settings: &LoggingSettings) -> Result<EnvFilter, TelemetryError> {
    Ok(EnvFilter::try_new(&settings.filter)?)
}

/// Layer di formattazione per il formato scelto, con lo scrittore indicato.
pub fn fmt_layer<S, W>(format: LogFormat, ansi: bool, writer: W) -> Box<dyn Layer<S> + Send + Sync>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    W: for<'w> MakeWriter<'w> + Send + Sync + 'static,
{
    match format {
        LogFormat::Pretty => fmt::layer().with_ansi(ansi).with_writer(writer).boxed(),
        LogFormat::Json => fmt::layer().json().with_writer(writer).boxed(),
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;
    use std::sync::{Arc, Mutex};

    use super::*;
    use crate::{SecretUrl, Settings};

    /// Scrittore in memoria per leggere i log prodotti nei test.
    #[derive(Clone, Default)]
    struct Buffer(Arc<Mutex<Vec<u8>>>);

    impl Write for Buffer {
        fn write(&mut self, data: &[u8]) -> std::io::Result<usize> {
            self.0.lock().unwrap().extend_from_slice(data);
            Ok(data.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    impl Buffer {
        fn contents(&self) -> String {
            String::from_utf8(self.0.lock().unwrap().clone()).unwrap()
        }
    }

    fn capture(format: LogFormat, log: impl FnOnce()) -> String {
        let buffer = Buffer::default();
        let writer = buffer.clone();
        let subscriber = tracing_subscriber::registry()
            .with(EnvFilter::new("info"))
            .with(fmt_layer(format, false, move || writer.clone()));
        tracing::subscriber::with_default(subscriber, log);
        buffer.contents()
    }

    #[test]
    fn formato_json_una_riga_per_evento() {
        let out = capture(LogFormat::Json, || {
            tracing::info!(block = 42_u64, "nuovo blocco")
        });
        let line = out.lines().next().unwrap();
        assert!(line.starts_with('{') && line.ends_with('}'), "{line}");
        assert!(line.contains("\"message\":\"nuovo blocco\""), "{line}");
        assert!(line.contains("\"block\":42"), "{line}");
    }

    #[test]
    fn formato_pretty_leggibile() {
        let out = capture(LogFormat::Pretty, || {
            tracing::info!(block = 42_u64, "nuovo blocco")
        });
        assert!(out.contains("nuovo blocco"), "{out}");
        assert!(out.contains("block=42"), "{out}");
    }

    #[test]
    fn i_segreti_non_compaiono_nei_log() {
        let key = "0123456789abcdef0123456789abcdef";
        let url = SecretUrl::parse(&format!("wss://polygon.example.com/v3/{key}")).unwrap();
        let mut settings = Settings::default();
        settings.rpc.ws_url = Some(url.clone());
        for format in [LogFormat::Pretty, LogFormat::Json] {
            let out = capture(format, || {
                tracing::info!(rpc = %url, "connessione");
                tracing::info!(?settings, "configurazione");
            });
            assert!(out.contains("polygon.example.com"), "{out}");
            assert!(!out.contains(key), "{out}");
        }
    }

    #[test]
    fn filtro_non_valido() {
        let settings = LoggingSettings {
            filter: "info,=[".to_owned(),
            ..LoggingSettings::default()
        };
        assert!(matches!(filter(&settings), Err(TelemetryError::Filter(_))));
    }
}
