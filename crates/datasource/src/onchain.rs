//! Adapter predefinito: eventi pubblici della blockchain Polygon.

use std::future::{Future, ready};

use sedette_types::BlockRange;

use crate::{DataSourceError, MarketDataSource, MarketEvent, SourceKind};

/// Sorgente basata sugli eventi on-chain degli exchange e dei contratti del
/// protocollo (indirizzi ed eventi in CONTRACTS.md). Scheletro: la lettura dei
/// log arriva in Fase 1.
#[derive(Debug, Clone, Default)]
pub struct OnChainSource {
    _private: (),
}

impl OnChainSource {
    /// Crea la sorgente.
    pub fn new() -> Self {
        Self::default()
    }
}

impl MarketDataSource for OnChainSource {
    fn name(&self) -> &'static str {
        "on-chain"
    }

    fn kind(&self) -> SourceKind {
        SourceKind::OnChain
    }

    fn market_events(
        &self,
        _range: BlockRange,
    ) -> impl Future<Output = Result<Vec<MarketEvent>, DataSourceError>> + Send {
        ready(Err(DataSourceError::NotImplemented {
            what: "lettura degli eventi on-chain",
            phase: "Fase 1",
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::now;

    #[test]
    fn scheletro_dichiara_la_fase() {
        let source = OnChainSource::new();
        assert_eq!(source.kind(), SourceKind::OnChain);
        let range = BlockRange::new(1, 2).unwrap();
        assert!(matches!(
            now(source.market_events(range)),
            Err(DataSourceError::NotImplemented {
                phase: "Fase 1",
                ..
            })
        ));
    }
}
