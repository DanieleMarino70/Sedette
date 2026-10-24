//! # sedette-chain
//!
//! Client alloy in sola lettura per Polygon: decodifica degli eventi, reorg e finalità.
//!
//! Non contiene codice di firma: nessuna chiave privata, nessuna transazione.
//! alloy è compilato senza feature di firma (vedi il `Cargo.toml` del workspace) e
//! `cargo deny` vieta i crate signer.
//!
//! Stato: in Fase 0 c'è la connessione a Polygon con la sottoscrizione ai nuovi
//! blocchi ([`ChainClient`]), il controllo di continuità delle teste
//! ([`HeadTracker`]) e il backoff di riconnessione ([`Backoff`]). La decodifica
//! degli eventi e la gestione dei reorg arrivano in Fase 1.

mod backoff;
mod client;
mod heads;

pub use backoff::Backoff;
pub use client::{ChainClient, ChainError};
pub use heads::{HeadEvent, HeadTracker};
