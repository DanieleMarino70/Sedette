//! Controllo di continuità dei nuovi blocchi ricevuti dalla sottoscrizione.

use sedette_types::BlockRef;

/// Come si collega un nuovo blocco a quello visto prima.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeadEvent {
    /// Primo blocco ricevuto: non c'è nulla con cui confrontarlo.
    First,
    /// Figlio diretto del blocco precedente.
    Next,
    /// Mancano dei blocchi tra il precedente e questo (per esempio dopo una
    /// riconnessione). `missing` è il numero di blocchi saltati.
    Gap {
        /// Blocchi mancanti.
        missing: u64,
    },
    /// Il nuovo blocco non discende dal precedente: possibile riorganizzazione.
    /// `depth` indica di quanti blocchi la nuova testa è tornata indietro
    /// rispetto a quella vista (0 se ha la stessa altezza o quella successiva).
    Reorg {
        /// Profondità stimata.
        depth: u64,
    },
}

/// Ricorda l'ultima testa vista e classifica le successive.
///
/// In Fase 0 serve solo a scrivere diagnostica nei log; la gestione vera delle
/// riorganizzazioni arriva in Fase 1.
#[derive(Debug, Clone, Default)]
pub struct HeadTracker {
    last: Option<BlockRef>,
}

impl HeadTracker {
    /// Crea il tracker senza blocchi visti.
    pub fn new() -> Self {
        Self::default()
    }

    /// Ultima testa vista.
    pub fn last(&self) -> Option<&BlockRef> {
        self.last.as_ref()
    }

    /// Registra `head` come nuova testa e dice come si collega alla precedente.
    pub fn observe(&mut self, head: BlockRef) -> HeadEvent {
        let event = match &self.last {
            None => HeadEvent::First,
            Some(prev) if head.follows(prev) => HeadEvent::Next,
            Some(prev) if head.number > prev.number.saturating_add(1) => HeadEvent::Gap {
                missing: head.number - prev.number - 1,
            },
            Some(prev) => HeadEvent::Reorg {
                depth: prev.number.saturating_sub(head.number.saturating_sub(1)),
            },
        };
        self.last = Some(head);
        event
    }
}

#[cfg(test)]
mod tests {
    use sedette_types::B256;

    use super::*;

    fn block(number: u64, hash: u8, parent: u8) -> BlockRef {
        BlockRef {
            number,
            hash: B256::repeat_byte(hash),
            parent_hash: B256::repeat_byte(parent),
            timestamp: 1_700_000_000,
        }
    }

    #[test]
    fn sequenza_regolare() {
        let mut t = HeadTracker::new();
        assert_eq!(t.observe(block(100, 1, 0)), HeadEvent::First);
        assert_eq!(t.observe(block(101, 2, 1)), HeadEvent::Next);
        assert_eq!(t.observe(block(102, 3, 2)), HeadEvent::Next);
        assert_eq!(t.last().map(|b| b.number), Some(102));
    }

    #[test]
    fn blocchi_mancanti() {
        let mut t = HeadTracker::new();
        t.observe(block(100, 1, 0));
        assert_eq!(t.observe(block(104, 5, 4)), HeadEvent::Gap { missing: 3 });
    }

    #[test]
    fn riorganizzazioni() {
        let mut t = HeadTracker::new();
        t.observe(block(100, 1, 0));
        // Stessa altezza successiva ma padre diverso.
        assert_eq!(t.observe(block(101, 2, 9)), HeadEvent::Reorg { depth: 0 });
        // Testa sostituita alla stessa altezza.
        assert_eq!(t.observe(block(101, 3, 1)), HeadEvent::Reorg { depth: 1 });
        // Ritorno indietro di due blocchi.
        t.observe(block(102, 4, 3));
        assert_eq!(t.observe(block(100, 5, 8)), HeadEvent::Reorg { depth: 3 });
    }
}
