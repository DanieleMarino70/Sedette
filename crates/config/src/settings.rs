//! Struttura della configurazione, caricamento e validazione.

use std::path::Path;

use config::{Config, Environment, File, FileFormat, Source};
use serde::Deserialize;

use crate::secret::SecretUrl;

/// Chain id di Polygon PoS mainnet, l'unica rete supportata.
pub const POLYGON_MAINNET_CHAIN_ID: u64 = 137;

/// Prefisso delle variabili d'ambiente (`SEDETTE_…`).
pub const ENV_PREFIX: &str = "SEDETTE";

/// Separatore tra sezione e chiave nelle variabili d'ambiente (`SEDETTE_RPC__WS_URL`).
const ENV_SEPARATOR: &str = "__";

/// Nome del file di configurazione predefinito, cercato nella directory corrente.
pub const DEFAULT_CONFIG_FILE: &str = "sedette.toml";

/// Chiavi che contengono segreti: ammesse solo da variabili d'ambiente.
const SECRET_KEYS: [(&str, &str); 2] = [
    ("rpc.ws_url", "SEDETTE_RPC__WS_URL"),
    ("rpc.http_url", "SEDETTE_RPC__HTTP_URL"),
];

/// Errori di caricamento e validazione della configurazione. I messaggi non
/// contengono mai il valore dei segreti.
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    /// Il file o le variabili d'ambiente non si leggono o non rispettano lo schema
    /// (tipo sbagliato, chiave sconosciuta, …).
    #[error("configurazione non valida: {0}")]
    Load(#[from] config::ConfigError),
    /// Un segreto è stato scritto nel file di configurazione.
    #[error(
        "{key} non va scritto in {file}: i segreti arrivano solo dalla variabile d'ambiente {env}"
    )]
    SecretInFile {
        /// Chiave trovata nel file.
        key: &'static str,
        /// File di configurazione.
        file: String,
        /// Variabile d'ambiente da usare al suo posto.
        env: &'static str,
    },
    /// La rete configurata non è Polygon PoS mainnet.
    #[error("chain.expected_chain_id = {0}: Sedette supporta solo Polygon PoS mainnet (137)")]
    UnsupportedChain(u64),
    /// Un valore è fuori dai limiti ammessi.
    #[error("{key}: {reason}")]
    Invalid {
        /// Chiave con il valore non valido.
        key: &'static str,
        /// Motivo.
        reason: String,
    },
    /// Manca un valore obbligatorio per il binario che lo richiede.
    #[error("manca {key}: impostala con la variabile d'ambiente {env} (vedi .env.example)")]
    Missing {
        /// Chiave mancante.
        key: &'static str,
        /// Variabile d'ambiente da impostare.
        env: &'static str,
    },
}

/// Configurazione completa di Sedette.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Settings {
    /// Rete blockchain.
    pub chain: ChainSettings,
    /// Endpoint RPC (solo da variabili d'ambiente).
    pub rpc: RpcSettings,
    /// Quando un blocco si considera definitivo.
    pub finality: FinalitySettings,
    /// Log.
    pub logging: LoggingSettings,
    /// Servizio di ingestione.
    pub ingestor: IngestorSettings,
}

/// Sezione `[chain]`.
#[derive(Debug, Clone, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ChainSettings {
    /// Chain id atteso dal provider RPC; deve essere 137.
    pub expected_chain_id: u64,
}

impl Default for ChainSettings {
    fn default() -> Self {
        Self {
            expected_chain_id: POLYGON_MAINNET_CHAIN_ID,
        }
    }
}

/// Sezione `[rpc]`. I valori contengono la chiave del provider e arrivano solo
/// da `SEDETTE_RPC__WS_URL` e `SEDETTE_RPC__HTTP_URL`.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct RpcSettings {
    /// Endpoint WebSocket (`ws://` o `wss://`).
    pub ws_url: Option<SecretUrl>,
    /// Endpoint HTTP (`http://` o `https://`).
    pub http_url: Option<SecretUrl>,
}

/// Sezione `[finality]`. I valori predefiniti sono provvisori: la regola di
/// finalità si decide nel piano della Fase 1.
#[derive(Debug, Clone, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct FinalitySettings {
    /// Profondità minima, in blocchi, per considerare definitivo un blocco.
    pub confirmations: u64,
    /// Considera definitivi anche i blocchi sotto il tag RPC `finalized`.
    pub use_finalized_tag: bool,
}

impl Default for FinalitySettings {
    fn default() -> Self {
        Self {
            confirmations: 32,
            use_finalized_tag: true,
        }
    }
}

/// Formato dei log.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    /// Testo leggibile, per lo sviluppo.
    #[default]
    Pretty,
    /// Una riga JSON per evento, per la raccolta dei log.
    Json,
}

/// Sezione `[logging]`.
#[derive(Debug, Clone, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct LoggingSettings {
    /// Formato di uscita.
    pub format: LogFormat,
    /// Direttive di filtro nella sintassi di `tracing_subscriber::EnvFilter`
    /// (per esempio `info,sedette_chain=debug`).
    pub filter: String,
}

impl Default for LoggingSettings {
    fn default() -> Self {
        Self {
            format: LogFormat::Pretty,
            filter: "info".to_owned(),
        }
    }
}

/// Sezione `[ingestor]`.
#[derive(Debug, Clone, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct IngestorSettings {
    /// Ogni quanti secondi leggere il blocco `finalized`.
    pub finalized_poll_secs: u64,
    /// Attesa iniziale prima di riconnettersi, in millisecondi.
    pub reconnect_initial_backoff_ms: u64,
    /// Attesa massima tra due tentativi di riconnessione, in secondi.
    pub reconnect_max_backoff_secs: u64,
}

impl Default for IngestorSettings {
    fn default() -> Self {
        Self {
            finalized_poll_secs: 30,
            reconnect_initial_backoff_ms: 500,
            reconnect_max_backoff_secs: 60,
        }
    }
}

impl Settings {
    /// Carica la configurazione dal file `file` (se indicato, deve esistere) e
    /// dalle variabili d'ambiente `env`, che hanno la precedenza. Le variabili
    /// senza prefisso `SEDETTE_` vengono ignorate.
    ///
    /// Le variabili si passano esplicitamente, invece di leggerle dal processo,
    /// così i binari possono unire `.env` e ambiente e i test non toccano lo
    /// stato globale.
    pub fn load<I>(file: Option<&Path>, env: I) -> Result<Self, ConfigError>
    where
        I: IntoIterator<Item = (String, String)>,
    {
        let file = file.map(|path| {
            (
                File::from(path).format(FileFormat::Toml).required(true),
                path.display().to_string(),
            )
        });
        Self::load_from(file, env)
    }

    fn load_from<S, I>(file: Option<(S, String)>, env: I) -> Result<Self, ConfigError>
    where
        S: Source + Clone + Send + Sync + 'static,
        I: IntoIterator<Item = (String, String)>,
    {
        let mut builder = Config::builder();
        if let Some((source, name)) = file {
            reject_secrets_in_file(source.clone(), &name)?;
            builder = builder.add_source(source);
        }
        let env = Environment::with_prefix(ENV_PREFIX)
            .prefix_separator("_")
            .separator(ENV_SEPARATOR)
            .source(Some(env.into_iter().collect()));
        let settings: Settings = builder.add_source(env).build()?.try_deserialize()?;
        settings.validate()?;
        Ok(settings)
    }

    /// Controlla i vincoli che lo schema da solo non esprime.
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.chain.expected_chain_id != POLYGON_MAINNET_CHAIN_ID {
            return Err(ConfigError::UnsupportedChain(self.chain.expected_chain_id));
        }
        if let Some(url) = &self.rpc.ws_url {
            check_scheme(url, "rpc.ws_url", &["ws", "wss"])?;
        }
        if let Some(url) = &self.rpc.http_url {
            check_scheme(url, "rpc.http_url", &["http", "https"])?;
        }
        check_range(
            "finality.confirmations",
            self.finality.confirmations,
            1,
            10_000,
        )?;
        check_range(
            "ingestor.finalized_poll_secs",
            self.ingestor.finalized_poll_secs,
            1,
            3_600,
        )?;
        check_range(
            "ingestor.reconnect_initial_backoff_ms",
            self.ingestor.reconnect_initial_backoff_ms,
            1,
            60_000,
        )?;
        check_range(
            "ingestor.reconnect_max_backoff_secs",
            self.ingestor.reconnect_max_backoff_secs,
            1,
            3_600,
        )?;
        if self.ingestor.reconnect_max_backoff_secs * 1_000
            < self.ingestor.reconnect_initial_backoff_ms
        {
            return Err(ConfigError::Invalid {
                key: "ingestor.reconnect_max_backoff_secs",
                reason: "deve essere almeno pari all'attesa iniziale".to_owned(),
            });
        }
        if self.logging.filter.trim().is_empty() {
            return Err(ConfigError::Invalid {
                key: "logging.filter",
                reason: "non può essere vuoto".to_owned(),
            });
        }
        Ok(())
    }

    /// Endpoint WebSocket, obbligatorio per i binari che seguono la chain in tempo reale.
    pub fn require_ws_url(&self) -> Result<&SecretUrl, ConfigError> {
        self.rpc.ws_url.as_ref().ok_or(ConfigError::Missing {
            key: "rpc.ws_url",
            env: "SEDETTE_RPC__WS_URL",
        })
    }
}

fn reject_secrets_in_file<S>(source: S, name: &str) -> Result<(), ConfigError>
where
    S: Source + Send + Sync + 'static,
{
    let file_only = Config::builder().add_source(source).build()?;
    for (key, env) in SECRET_KEYS {
        if file_only.get::<config::Value>(key).is_ok() {
            return Err(ConfigError::SecretInFile {
                key,
                file: name.to_owned(),
                env,
            });
        }
    }
    Ok(())
}

fn check_scheme(url: &SecretUrl, key: &'static str, allowed: &[&str]) -> Result<(), ConfigError> {
    let scheme = url.scheme();
    if allowed.contains(&scheme.as_str()) {
        Ok(())
    } else {
        Err(ConfigError::Invalid {
            key,
            reason: format!(
                "schema {scheme} non ammesso (ammessi: {})",
                allowed.join(", ")
            ),
        })
    }
}

fn check_range(key: &'static str, value: u64, min: u64, max: u64) -> Result<(), ConfigError> {
    if (min..=max).contains(&value) {
        Ok(())
    } else {
        Err(ConfigError::Invalid {
            key,
            reason: format!("{value} fuori dall'intervallo ammesso {min}..={max}"),
        })
    }
}

#[cfg(test)]
mod tests {
    use config::FileSourceString;

    use super::*;

    const KEY: &str = "0123456789abcdef0123456789abcdef";

    fn env(vars: &[(&str, &str)]) -> Vec<(String, String)> {
        vars.iter()
            .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
            .collect()
    }

    fn load_toml(toml: &str, vars: &[(&str, &str)]) -> Result<Settings, ConfigError> {
        let source = File::from_str(toml, FileFormat::Toml);
        Settings::load_from(Some((source, "test.toml".to_owned())), env(vars))
    }

    fn load_env(vars: &[(&str, &str)]) -> Result<Settings, ConfigError> {
        Settings::load_from(
            None::<(File<FileSourceString, FileFormat>, String)>,
            env(vars),
        )
    }

    #[test]
    fn senza_file_ne_variabili_usa_i_default() {
        let s = load_env(&[]).unwrap();
        assert_eq!(s.chain.expected_chain_id, 137);
        assert!(s.rpc.ws_url.is_none());
        assert_eq!(s.logging.format, LogFormat::Pretty);
        assert_eq!(s.finality.confirmations, 32);
    }

    #[test]
    fn le_variabili_d_ambiente_vincono_sul_file() {
        let toml =
            "[logging]\nformat = \"pretty\"\nfilter = \"warn\"\n[finality]\nconfirmations = 64\n";
        let s = load_toml(toml, &[("SEDETTE_LOGGING__FORMAT", "json")]).unwrap();
        assert_eq!(s.logging.format, LogFormat::Json);
        assert_eq!(s.logging.filter, "warn");
        assert_eq!(s.finality.confirmations, 64);
    }

    #[test]
    fn i_numeri_arrivano_anche_come_stringhe_dall_ambiente() {
        let s = load_env(&[("SEDETTE_FINALITY__CONFIRMATIONS", "10")]).unwrap();
        assert_eq!(s.finality.confirmations, 10);
    }

    #[test]
    fn le_variabili_senza_prefisso_sono_ignorate() {
        let s = load_env(&[("POSTGRES_PASSWORD", "x"), ("RUST_LOG", "trace")]).unwrap();
        assert_eq!(s.logging.filter, "info");
    }

    #[test]
    fn l_url_rpc_arriva_dall_ambiente_ed_e_mascherato() {
        let url = format!("wss://polygon.example.com/ws/v3/{KEY}");
        let s = load_env(&[("SEDETTE_RPC__WS_URL", url.as_str())]).unwrap();
        assert_eq!(s.require_ws_url().unwrap().expose(), url);
        assert!(!format!("{s:?}").contains(KEY));
    }

    #[test]
    fn rifiuta_i_segreti_nel_file() {
        let toml = format!("[rpc]\nws_url = \"wss://polygon.example.com/{KEY}\"\n");
        let err = load_toml(&toml, &[]).unwrap_err();
        assert!(matches!(
            err,
            ConfigError::SecretInFile {
                key: "rpc.ws_url",
                ..
            }
        ));
        assert!(!err.to_string().contains(KEY));
    }

    #[test]
    fn rifiuta_reti_diverse_da_polygon() {
        let err = load_env(&[("SEDETTE_CHAIN__EXPECTED_CHAIN_ID", "1")]).unwrap_err();
        assert!(matches!(err, ConfigError::UnsupportedChain(1)));
    }

    #[test]
    fn rifiuta_schemi_sbagliati() {
        let err = load_env(&[("SEDETTE_RPC__WS_URL", "https://polygon.example.com")]).unwrap_err();
        assert!(matches!(
            err,
            ConfigError::Invalid {
                key: "rpc.ws_url",
                ..
            }
        ));

        let err = load_env(&[("SEDETTE_RPC__HTTP_URL", "wss://polygon.example.com")]).unwrap_err();
        assert!(matches!(
            err,
            ConfigError::Invalid {
                key: "rpc.http_url",
                ..
            }
        ));
    }

    #[test]
    fn rifiuta_url_malformati_senza_mostrarli() {
        let err = load_env(&[("SEDETTE_RPC__WS_URL", &format!("senza-schema/{KEY}"))]).unwrap_err();
        assert!(matches!(err, ConfigError::Load(_)));
        assert!(!err.to_string().contains(KEY));
    }

    #[test]
    fn rifiuta_valori_fuori_intervallo() {
        let err = load_env(&[("SEDETTE_FINALITY__CONFIRMATIONS", "0")]).unwrap_err();
        assert!(matches!(
            err,
            ConfigError::Invalid {
                key: "finality.confirmations",
                ..
            }
        ));

        let err = load_env(&[
            ("SEDETTE_INGESTOR__RECONNECT_INITIAL_BACKOFF_MS", "5000"),
            ("SEDETTE_INGESTOR__RECONNECT_MAX_BACKOFF_SECS", "2"),
        ])
        .unwrap_err();
        assert!(matches!(
            err,
            ConfigError::Invalid {
                key: "ingestor.reconnect_max_backoff_secs",
                ..
            }
        ));
    }

    #[test]
    fn rifiuta_chiavi_sconosciute() {
        assert!(matches!(
            load_toml("[loging]\nformat = \"json\"\n", &[]),
            Err(ConfigError::Load(_))
        ));
        assert!(matches!(
            load_env(&[("SEDETTE_RPC__WSURL", "wss://x.example.com")]),
            Err(ConfigError::Load(_))
        ));
    }

    #[test]
    fn segnala_l_url_websocket_mancante() {
        let err = load_env(&[]).unwrap().require_ws_url().unwrap_err();
        assert!(err.to_string().contains("SEDETTE_RPC__WS_URL"));
    }

    #[test]
    fn il_file_versionato_e_valido() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../sedette.toml");
        let s = Settings::load(Some(&path), Vec::new()).unwrap();
        assert_eq!(s.chain.expected_chain_id, POLYGON_MAINNET_CHAIN_ID);
    }
}
