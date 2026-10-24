//! Connessione in sola lettura a un nodo Polygon tramite alloy.

use alloy::eips::BlockNumberOrTag;
use alloy::providers::{DynProvider, Provider, ProviderBuilder, WsConnect};
use alloy::rpc::types::Header;
use futures_util::{Stream, StreamExt};
use sedette_config::SecretUrl;
use sedette_types::BlockRef;

/// Errori del client. I messaggi sono ripuliti dall'URL dell'endpoint, che può
/// contenere la chiave del provider.
#[derive(Debug, thiserror::Error)]
pub enum ChainError {
    /// Connessione al nodo non riuscita.
    #[error("connessione a {endpoint} non riuscita: {message}")]
    Connect {
        /// Endpoint mascherato.
        endpoint: String,
        /// Causa, senza segreti.
        message: String,
    },
    /// Il nodo appartiene a un'altra rete.
    #[error("il nodo {endpoint} è sulla chain {actual}, attesa {expected}")]
    WrongChain {
        /// Endpoint mascherato.
        endpoint: String,
        /// Chain id atteso.
        expected: u64,
        /// Chain id restituito dal nodo.
        actual: u64,
    },
    /// Chiamata RPC non riuscita.
    #[error("chiamata {method} non riuscita: {message}")]
    Rpc {
        /// Metodo JSON-RPC.
        method: &'static str,
        /// Causa, senza segreti.
        message: String,
    },
}

impl ChainError {
    /// Vero se riprovare non serve (per esempio la rete è sbagliata).
    pub fn is_fatal(&self) -> bool {
        matches!(self, ChainError::WrongChain { .. })
    }
}

/// Client in sola lettura verso un nodo Polygon PoS.
#[derive(Debug, Clone)]
pub struct ChainClient {
    provider: DynProvider,
    endpoint: SecretUrl,
}

impl ChainClient {
    /// Si connette all'endpoint WebSocket e verifica che il nodo sia sulla chain
    /// `expected_chain_id`.
    pub async fn connect_ws(
        endpoint: &SecretUrl,
        expected_chain_id: u64,
    ) -> Result<Self, ChainError> {
        let ws = WsConnect::new(endpoint.expose());
        let provider = ProviderBuilder::default()
            .connect_ws(ws)
            .await
            .map_err(|e| ChainError::Connect {
                endpoint: endpoint.redacted().to_owned(),
                message: sanitize(&e.to_string(), endpoint),
            })?
            .erased();
        let client = Self {
            provider,
            endpoint: endpoint.clone(),
        };

        let actual = client
            .provider
            .get_chain_id()
            .await
            .map_err(|e| client.rpc_error("eth_chainId", &e))?;
        if actual != expected_chain_id {
            return Err(ChainError::WrongChain {
                endpoint: endpoint.redacted().to_owned(),
                expected: expected_chain_id,
                actual,
            });
        }
        Ok(client)
    }

    /// Endpoint, nella forma mascherata adatta ai log.
    pub fn endpoint(&self) -> &SecretUrl {
        &self.endpoint
    }

    /// Flusso dei nuovi blocchi (`eth_subscribe("newHeads")`). Contiene solo gli
    /// header, quindi non decodifica le transazioni del blocco. Il flusso
    /// termina se la sottoscrizione si chiude.
    pub async fn subscribe_heads(
        &self,
    ) -> Result<impl Stream<Item = BlockRef> + use<>, ChainError> {
        let subscription = self
            .provider
            .subscribe_blocks()
            .await
            .map_err(|e| self.rpc_error("eth_subscribe", &e))?;
        Ok(subscription.into_stream().map(|header| block_ref(&header)))
    }

    /// Altezza dell'ultimo blocco.
    pub async fn latest_number(&self) -> Result<u64, ChainError> {
        self.provider
            .get_block_number()
            .await
            .map_err(|e| self.rpc_error("eth_blockNumber", &e))
    }

    /// Ultimo blocco con finalità deterministica (tag `finalized`), se il nodo
    /// lo fornisce.
    pub async fn finalized(&self) -> Result<Option<BlockRef>, ChainError> {
        let block = self
            .provider
            .get_block_by_number(BlockNumberOrTag::Finalized)
            .await
            .map_err(|e| self.rpc_error("eth_getBlockByNumber", &e))?;
        Ok(block.map(|b| block_ref(&b.header)))
    }

    fn rpc_error(&self, method: &'static str, error: &dyn std::fmt::Display) -> ChainError {
        ChainError::Rpc {
            method,
            message: sanitize(&error.to_string(), &self.endpoint),
        }
    }
}

fn block_ref(header: &Header) -> BlockRef {
    BlockRef {
        number: header.number,
        hash: header.hash,
        parent_hash: header.parent_hash,
        timestamp: header.timestamp,
    }
}

/// Toglie dal messaggio l'URL completo dell'endpoint e la sua parte segreta
/// (percorso, query, credenziali), sostituendoli con la forma mascherata.
fn sanitize(message: &str, endpoint: &SecretUrl) -> String {
    let full = endpoint.expose();
    let mut clean = message.replace(full, endpoint.redacted());
    if let Some((_, rest)) = full.split_once("://") {
        let host = endpoint
            .redacted()
            .split_once("://")
            .map_or("", |(_, h)| h.trim_end_matches("/***"));
        // Qualsiasi frammento dopo l'host può contenere la chiave: si toglie la
        // coda intera e poi ogni suo segmento lungo, che può comparire da solo.
        if let Some(pos) = rest.find(host) {
            let secret_tail = rest[pos + host.len()..].trim_start_matches('/');
            if secret_tail.len() >= 8 {
                clean = clean.replace(secret_tail, "***");
            }
            for segment in secret_tail.split(['/', '?', '&', '=', '#']) {
                if segment.len() >= 8 {
                    clean = clean.replace(segment, "***");
                }
            }
        }
        if let Some((userinfo, _)) = rest.split_once('@')
            && !userinfo.contains('/')
        {
            for part in std::iter::once(userinfo).chain(userinfo.split(':')) {
                if part.len() >= 4 {
                    clean = clean.replace(part, "***");
                }
            }
        }
    }
    clean
}

#[cfg(test)]
mod tests {
    use super::*;

    const KEY: &str = "0123456789abcdef0123456789abcdef";

    #[test]
    fn sanitize_toglie_url_e_chiave() {
        let url = SecretUrl::parse(&format!("wss://polygon.example.com/ws/v3/{KEY}")).unwrap();
        let msg = format!(
            "HTTP error 401 for wss://polygon.example.com/ws/v3/{KEY}: invalid key {KEY}; path ws/v3/{KEY}"
        );
        let clean = sanitize(&msg, &url);
        assert!(!clean.contains(KEY), "{clean}");
        assert!(clean.contains("wss://polygon.example.com/***"), "{clean}");
    }

    #[test]
    fn sanitize_toglie_le_credenziali() {
        let url = SecretUrl::parse(&format!("wss://utente:{KEY}@polygon.example.com")).unwrap();
        let clean = sanitize(&format!("auth failed for utente:{KEY}"), &url);
        assert!(!clean.contains(KEY), "{clean}");
    }

    #[test]
    fn sanitize_lascia_intatti_i_messaggi_senza_segreti() {
        let url = SecretUrl::parse("wss://polygon-bor-rpc.publicnode.com").unwrap();
        assert_eq!(sanitize("connection reset", &url), "connection reset");
    }

    #[test]
    fn solo_la_rete_sbagliata_e_fatale() {
        let wrong = ChainError::WrongChain {
            endpoint: "wss://x".into(),
            expected: 137,
            actual: 1,
        };
        let rpc = ChainError::Rpc {
            method: "eth_chainId",
            message: "timeout".into(),
        };
        assert!(wrong.is_fatal());
        assert!(!rpc.is_fatal());
    }
}
