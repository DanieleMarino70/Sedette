//! # sedette-types
//!
//! Tipi condivisi: mercato, esito, trade, posizione, wallet.
//!
//! Contiene solo tipi e funzioni pure, senza I/O, così ogni altro crate può dipenderne.
//!
//! Stato: in Fase 0 ci sono i tipi di base (blocchi, identificatori, lato di un
//! ordine). I tipi di dominio completi (mercato, trade, posizione) arrivano in
//! Fase 1, quando gli eventi on-chain saranno decodificati.

mod block;
mod ids;

pub use alloy_primitives::{Address, B256, U256};
pub use block::{BlockRange, BlockRef, InvalidBlockRange};
pub use ids::{ConditionId, QuestionId, Side, TokenId, UnknownSide, WalletAddress};
