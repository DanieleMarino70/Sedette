//! # sedette-datasource
//!
//! Trait [`MarketDataSource`] e relativi adapter.
//!
//! Il resto del sistema legge i dati di mercato solo attraverso questo trait, così
//! la provenienza dei dati resta sostituibile senza riscrivere l'architettura.
//!
//! | Adapter | Stato |
//! |---|---|
//! | [`OnChainSource`] | Predefinito. Dati pubblici della blockchain Polygon tramite RPC. Implementato in Fase 1. |
//! | `ClobSource` | **Non implementato e disabilitato**, dietro la feature `clob-adapter`. |
//!
//! L'adapter per il CLOB di Polymarket esiste solo per non dover cambiare
//! l'architettura in futuro. **Va attivato solo in una giurisdizione in cui l'uso
//! è legale.** Non contiene client di rete né URL: anche compilato con la
//! feature, restituisce sempre [`DataSourceError::Disabled`].
//!
//! Stato: in Fase 0 c'è la bozza del trait, da rivedere in Fase 1 quando gli
//! eventi on-chain saranno decodificati.

use std::future::Future;

use sedette_types::BlockRange;

#[cfg(feature = "clob-adapter")]
mod clob;
mod onchain;

#[cfg(feature = "clob-adapter")]
pub use clob::ClobSource;
pub use onchain::OnChainSource;

/// Tipo di sorgente dei dati di mercato.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SourceKind {
    /// Eventi pubblici della blockchain, letti tramite un provider RPC.
    OnChain,
    /// CLOB di Polymarket: disabilitato, vedi la documentazione del crate.
    Clob,
}

/// Evento di mercato prodotto da una sorgente.
///
/// Bozza: le varianti (trade, registrazione dei token, split/merge/redeem,
/// inizializzazione e risoluzione delle domande) si definiscono in Fase 1.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum MarketEvent {}

/// Errori di una sorgente di dati.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DataSourceError {
    /// La sorgente è disabilitata per scelta e non va usata.
    #[error("la sorgente {source_name} è disabilitata")]
    Disabled {
        /// Nome della sorgente.
        source_name: &'static str,
    },
    /// La funzionalità arriva in una fase successiva.
    #[error("{what}: non ancora implementato ({phase})")]
    NotImplemented {
        /// Cosa manca.
        what: &'static str,
        /// Fase in cui arriva.
        phase: &'static str,
    },
}

/// Sorgente di eventi di mercato.
pub trait MarketDataSource {
    /// Nome leggibile, per i log.
    fn name(&self) -> &'static str;

    /// Tipo di sorgente.
    fn kind(&self) -> SourceKind;

    /// Eventi di mercato nei blocchi `range`, in ordine di blocco e di log.
    fn market_events(
        &self,
        range: BlockRange,
    ) -> impl Future<Output = Result<Vec<MarketEvent>, DataSourceError>> + Send;
}

/// Tipi di sorgente compilati in questo binario. Senza la feature
/// `clob-adapter` c'è solo [`SourceKind::OnChain`].
pub fn compiled_sources() -> &'static [SourceKind] {
    if cfg!(feature = "clob-adapter") {
        &[SourceKind::OnChain, SourceKind::Clob]
    } else {
        &[SourceKind::OnChain]
    }
}

#[cfg(test)]
pub(crate) mod test_util {
    use std::future::Future;
    use std::pin::pin;
    use std::task::{Context, Poll, Waker};

    /// Esegue un future che si completa al primo poll, senza runtime asincrono.
    pub(crate) fn now<F: Future>(future: F) -> F::Output {
        let mut cx = Context::from_waker(Waker::noop());
        match pin!(future).poll(&mut cx) {
            Poll::Ready(output) => output,
            Poll::Pending => panic!("il future non è pronto"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(not(feature = "clob-adapter"))]
    #[test]
    fn la_build_predefinita_non_include_il_clob() {
        assert_eq!(compiled_sources(), &[SourceKind::OnChain]);
    }

    #[cfg(feature = "clob-adapter")]
    #[test]
    fn con_la_feature_il_clob_e_compilato_ma_disabilitato() {
        assert!(compiled_sources().contains(&SourceKind::Clob));
    }
}
