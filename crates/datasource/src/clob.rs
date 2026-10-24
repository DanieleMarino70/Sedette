//! Adapter per il CLOB di Polymarket: **non implementato e disabilitato**.
//!
//! Esiste solo per non dover cambiare l'architettura in futuro. **Da attivare
//! solo in una giurisdizione in cui l'uso è legale**, e solo dopo una decisione
//! registrata in un ADR.
//!
//! Questo modulo non contiene client di rete, URL né credenziali: ogni chiamata
//! restituisce [`DataSourceError::Disabled`]. Il guardrail di CI rifiuta
//! qualsiasi riferimento ai domini di Polymarket nel codice.

use std::future::{Future, ready};

use sedette_types::BlockRange;

use crate::{DataSourceError, MarketDataSource, MarketEvent, SourceKind};

/// Adapter disabilitato per il CLOB di Polymarket.
#[derive(Debug, Clone, Default)]
pub struct ClobSource {
    _private: (),
}

impl ClobSource {
    /// Crea l'adapter. Non apre connessioni e non ne aprirà.
    pub fn new() -> Self {
        Self::default()
    }
}

impl MarketDataSource for ClobSource {
    fn name(&self) -> &'static str {
        "clob (disabilitato)"
    }

    fn kind(&self) -> SourceKind {
        SourceKind::Clob
    }

    fn market_events(
        &self,
        _range: BlockRange,
    ) -> impl Future<Output = Result<Vec<MarketEvent>, DataSourceError>> + Send {
        ready(Err(DataSourceError::Disabled {
            source_name: self.name(),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::now;

    #[test]
    fn restituisce_sempre_disabled() {
        let source = ClobSource::new();
        let range = BlockRange::new(1, 1).unwrap();
        assert_eq!(
            now(source.market_events(range)),
            Err(DataSourceError::Disabled {
                source_name: "clob (disabilitato)"
            })
        );
    }
}
