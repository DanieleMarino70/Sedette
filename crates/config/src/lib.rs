//! # sedette-config
//!
//! Caricamento e validazione della configurazione (sedette.toml e variabili SEDETTE_).
//!
//! La configurazione si legge da due fonti, in ordine di precedenza crescente:
//!
//! 1. il file `sedette.toml`, versionato e **senza segreti**;
//! 2. le variabili d'ambiente con prefisso `SEDETTE_`, dove `__` separa le sezioni
//!    (per esempio `SEDETTE_RPC__WS_URL` imposta `rpc.ws_url`).
//!
//! I segreti, come gli URL RPC con chiave, arrivano solo da variabili d'ambiente:
//! se compaiono nel file, il caricamento fallisce. Il tipo [`SecretUrl`] impedisce
//! che finiscano nei log.
//!
//! Il modulo [`telemetry`] inizializza `tracing` secondo la sezione `[logging]`.

mod secret;
mod settings;
pub mod telemetry;

pub use secret::{InvalidUrl, SecretUrl};
pub use settings::{
    ChainSettings, ConfigError, DEFAULT_CONFIG_FILE, ENV_PREFIX, FinalitySettings,
    IngestorSettings, LogFormat, LoggingSettings, POLYGON_MAINNET_CHAIN_ID, RpcSettings, Settings,
};
