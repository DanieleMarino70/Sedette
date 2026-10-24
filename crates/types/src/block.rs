//! Riferimenti ai blocchi e intervalli di blocchi.

use std::fmt;

use alloy_primitives::B256;

/// Riferimento a un blocco della chain: quanto serve per seguirne la sequenza
/// e accorgersi di riorganizzazioni.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockRef {
    /// Altezza del blocco.
    pub number: u64,
    /// Hash del blocco.
    pub hash: B256,
    /// Hash del blocco padre.
    pub parent_hash: B256,
    /// Timestamp del blocco, in secondi Unix.
    pub timestamp: u64,
}

impl BlockRef {
    /// Vero se questo blocco è il figlio diretto di `prev`: altezza successiva
    /// e `parent_hash` uguale all'hash di `prev`.
    pub fn follows(&self, prev: &BlockRef) -> bool {
        prev.number.checked_add(1) == Some(self.number) && self.parent_hash == prev.hash
    }
}

impl fmt::Display for BlockRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{} ({})", self.number, self.hash)
    }
}

/// Intervallo di blocchi, estremi inclusi.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockRange {
    from: u64,
    to: u64,
}

/// Intervallo con l'inizio dopo la fine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("intervallo di blocchi non valido: {from} > {to}")]
pub struct InvalidBlockRange {
    /// Primo blocco richiesto.
    pub from: u64,
    /// Ultimo blocco richiesto.
    pub to: u64,
}

impl BlockRange {
    /// Crea l'intervallo `from..=to`.
    pub fn new(from: u64, to: u64) -> Result<Self, InvalidBlockRange> {
        if from <= to {
            Ok(Self { from, to })
        } else {
            Err(InvalidBlockRange { from, to })
        }
    }

    /// Primo blocco, incluso.
    pub fn from(&self) -> u64 {
        self.from
    }

    /// Ultimo blocco, incluso.
    pub fn to(&self) -> u64 {
        self.to
    }

    /// Numero di blocchi nell'intervallo.
    pub fn len(&self) -> u64 {
        self.to - self.from + 1
    }

    /// Un intervallo valido contiene sempre almeno un blocco.
    pub fn is_empty(&self) -> bool {
        false
    }

    /// Vero se `number` è nell'intervallo.
    pub fn contains(&self, number: u64) -> bool {
        (self.from..=self.to).contains(&number)
    }
}

#[cfg(test)]
mod tests {
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
    fn riconosce_il_figlio_diretto() {
        let a = block(100, 0xaa, 0x99);
        let b = block(101, 0xbb, 0xaa);
        assert!(b.follows(&a));
    }

    #[test]
    fn segnala_buchi_e_reorg() {
        let a = block(100, 0xaa, 0x99);
        // Altezza giusta ma padre diverso: riorganizzazione.
        assert!(!block(101, 0xbb, 0xcc).follows(&a));
        // Padre giusto ma altezza saltata: dato incoerente.
        assert!(!block(102, 0xbb, 0xaa).follows(&a));
        // Stesso blocco ripetuto.
        assert!(!a.follows(&a));
        // Nessun overflow all'altezza massima.
        assert!(!block(0, 0xbb, 0xaa).follows(&block(u64::MAX, 0xaa, 0x99)));
    }

    #[test]
    fn intervallo_inclusivo() {
        let r = BlockRange::new(10, 12).unwrap();
        assert_eq!(r.len(), 3);
        assert!(r.contains(10) && r.contains(12) && !r.contains(13));
        assert_eq!(BlockRange::new(5, 5).unwrap().len(), 1);
        assert_eq!(
            BlockRange::new(12, 10),
            Err(InvalidBlockRange { from: 12, to: 10 })
        );
    }
}
