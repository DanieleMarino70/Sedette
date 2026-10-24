//! URL che contengono segreti e non devono mai finire nei log.

use std::fmt;

use serde::{Deserialize, Deserializer};

/// URL che può contenere una chiave nel percorso, nella query o nelle credenziali,
/// come gli endpoint RPC dei provider (`wss://…/v3/<chiave>`).
///
/// `Debug` e `Display` mostrano solo schema e host, seguiti da `/***` se il resto
/// è stato nascosto. Il valore completo si ottiene solo con [`SecretUrl::expose`],
/// da usare esclusivamente per aprire la connessione.
#[derive(Clone, PartialEq, Eq)]
pub struct SecretUrl {
    value: String,
    redacted: String,
    scheme_len: usize,
}

/// Errore di validazione di un [`SecretUrl`]. Il messaggio non contiene mai
/// l'URL, che potrebbe includere una chiave.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum InvalidUrl {
    /// Manca `schema://`.
    #[error("URL non valido: manca lo schema (per esempio wss://)")]
    MissingScheme,
    /// Lo schema contiene caratteri non ammessi.
    #[error("URL non valido: schema non ammesso")]
    BadScheme,
    /// Manca l'host dopo `schema://`.
    #[error("URL non valido: manca l'host")]
    MissingHost,
    /// L'URL contiene spazi o caratteri di controllo.
    #[error("URL non valido: contiene spazi o caratteri di controllo")]
    Whitespace,
}

impl SecretUrl {
    /// Valida l'URL e prepara la sua forma mascherata.
    pub fn parse(value: &str) -> Result<Self, InvalidUrl> {
        if value.chars().any(|c| c.is_whitespace() || c.is_control()) {
            return Err(InvalidUrl::Whitespace);
        }
        let (scheme, rest) = value.split_once("://").ok_or(InvalidUrl::MissingScheme)?;
        let scheme_ok = scheme
            .chars()
            .next()
            .is_some_and(|c| c.is_ascii_alphabetic())
            && scheme
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '-' | '.'));
        if !scheme_ok {
            return Err(InvalidUrl::BadScheme);
        }

        let authority_end = rest.find(['/', '?', '#']).unwrap_or(rest.len());
        let authority = &rest[..authority_end];
        let (userinfo, host) = match authority.rsplit_once('@') {
            Some((user, host)) => (Some(user), host),
            None => (None, authority),
        };
        if host.is_empty() || host.starts_with(':') {
            return Err(InvalidUrl::MissingHost);
        }

        let tail = rest[authority_end..].trim_start_matches('/');
        let hidden = userinfo.is_some() || !tail.is_empty();
        let redacted = if hidden {
            format!("{scheme}://{host}/***")
        } else {
            format!("{scheme}://{host}")
        };

        Ok(Self {
            value: value.to_owned(),
            redacted,
            scheme_len: scheme.len(),
        })
    }

    /// Valore completo, chiave compresa. Da usare solo per aprire la connessione,
    /// mai per log o messaggi di errore.
    pub fn expose(&self) -> &str {
        &self.value
    }

    /// Schema in minuscolo (`ws`, `wss`, `http`, `https`, …).
    pub fn scheme(&self) -> String {
        self.value[..self.scheme_len].to_ascii_lowercase()
    }

    /// Forma mascherata: schema e host, senza percorso, query né credenziali.
    pub fn redacted(&self) -> &str {
        &self.redacted
    }
}

impl fmt::Debug for SecretUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SecretUrl({})", self.redacted)
    }
}

impl fmt::Display for SecretUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.redacted)
    }
}

impl<'de> Deserialize<'de> for SecretUrl {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        SecretUrl::parse(&value).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const KEY: &str = "0123456789abcdef0123456789abcdef";

    #[test]
    fn nasconde_la_chiave_nel_percorso() {
        let url = SecretUrl::parse(&format!("wss://polygon.example.com/ws/v3/{KEY}")).unwrap();
        assert_eq!(url.redacted(), "wss://polygon.example.com/***");
        assert_eq!(url.to_string(), "wss://polygon.example.com/***");
        assert_eq!(
            format!("{url:?}"),
            "SecretUrl(wss://polygon.example.com/***)"
        );
        assert!(!format!("{url:?}{url}").contains(KEY));
    }

    #[test]
    fn nasconde_query_e_credenziali() {
        let query = SecretUrl::parse(&format!("https://rpc.example.org?dkey={KEY}")).unwrap();
        assert_eq!(query.redacted(), "https://rpc.example.org/***");

        let userinfo =
            SecretUrl::parse(&format!("https://user:{KEY}@rpc.example.org:8545")).unwrap();
        assert_eq!(userinfo.redacted(), "https://rpc.example.org:8545/***");
        assert!(!userinfo.to_string().contains(KEY));
    }

    #[test]
    fn url_senza_segreti_resta_leggibile() {
        let url = SecretUrl::parse("wss://polygon-bor-rpc.publicnode.com").unwrap();
        assert_eq!(url.redacted(), "wss://polygon-bor-rpc.publicnode.com");
        assert_eq!(url.scheme(), "wss");
    }

    #[test]
    fn expose_restituisce_il_valore_completo() {
        let raw = format!("wss://polygon.example.com/{KEY}");
        let url = SecretUrl::parse(&raw).unwrap();
        assert_eq!(url.expose(), raw);
    }

    #[test]
    fn rifiuta_url_non_validi() {
        assert_eq!(
            SecretUrl::parse("polygon.example.com"),
            Err(InvalidUrl::MissingScheme)
        );
        assert_eq!(SecretUrl::parse("1ws://host"), Err(InvalidUrl::BadScheme));
        assert_eq!(
            SecretUrl::parse("wss:///percorso"),
            Err(InvalidUrl::MissingHost)
        );
        assert_eq!(
            SecretUrl::parse("wss://user@:443"),
            Err(InvalidUrl::MissingHost)
        );
        assert_eq!(
            SecretUrl::parse("wss://host/ chiave"),
            Err(InvalidUrl::Whitespace)
        );
    }

    #[test]
    fn gli_errori_non_contengono_l_url() {
        let raw = format!("senza-schema/{KEY}");
        let err = SecretUrl::parse(&raw).unwrap_err();
        assert!(!err.to_string().contains(KEY));
    }
}
