//! Identificatori del protocollo e lato di un ordine.
//!
//! Gli identificatori sono newtype distinti, anche quando condividono la
//! rappresentazione (`ConditionId` e `QuestionId` sono entrambi 32 byte), così
//! il compilatore impedisce di scambiarli.

use std::fmt;

use alloy_primitives::{Address, B256, U256};

macro_rules! id_type {
    ($(#[$doc:meta])* $name:ident($inner:ty)) => {
        $(#[$doc])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
        pub struct $name(pub $inner);

        impl From<$inner> for $name {
            fn from(value: $inner) -> Self {
                Self(value)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Display::fmt(&self.0, f)
            }
        }
    };
}

id_type!(
    /// Identificatore di una condizione del Conditional Tokens Framework
    /// (`conditionId`, 32 byte).
    ConditionId(B256)
);

id_type!(
    /// Identificatore di una domanda dell'oracolo (`questionID`, 32 byte),
    /// da cui si ricava la condizione.
    QuestionId(B256)
);

id_type!(
    /// Identificatore ERC-1155 di un token di esito (`positionId` / `tokenId`).
    TokenId(U256)
);

id_type!(
    /// Indirizzo di un wallet che opera sui mercati.
    WalletAddress(Address)
);

/// Lato di un ordine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Side {
    /// Acquisto di token di esito.
    Buy,
    /// Vendita di token di esito.
    Sell,
}

/// Valore del lato non previsto dal protocollo.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("lato dell'ordine sconosciuto: {0}")]
pub struct UnknownSide(pub u8);

impl TryFrom<u8> for Side {
    type Error = UnknownSide;

    /// Codifica degli exchange: `0` = acquisto, `1` = vendita (fonte in CONTRACTS.md).
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Side::Buy),
            1 => Ok(Side::Sell),
            other => Err(UnknownSide(other)),
        }
    }
}

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Side::Buy => "buy",
            Side::Sell => "sell",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lato_dalla_codifica_on_chain() {
        assert_eq!(Side::try_from(0), Ok(Side::Buy));
        assert_eq!(Side::try_from(1), Ok(Side::Sell));
        assert_eq!(Side::try_from(2), Err(UnknownSide(2)));
    }

    #[test]
    fn gli_id_si_stampano_come_il_valore_interno() {
        let id = ConditionId(B256::repeat_byte(0xab));
        assert_eq!(id.to_string(), B256::repeat_byte(0xab).to_string());
        assert_eq!(TokenId(U256::from(42u64)).to_string(), "42");
    }
}
