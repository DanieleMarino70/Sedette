//! Test contro un nodo Polygon reale. Sono `#[ignore]`: la CI non ha segreti e
//! non li esegue. In locale:
//!
//! ```text
//! SEDETTE_RPC__WS_URL=wss://… cargo test -p sedette-chain --test live -- --ignored
//! ```

use std::time::Duration;

use futures_util::StreamExt;
use sedette_chain::ChainClient;
use sedette_config::{POLYGON_MAINNET_CHAIN_ID, SecretUrl};

fn endpoint() -> SecretUrl {
    let raw = std::env::var("SEDETTE_RPC__WS_URL")
        .expect("imposta SEDETTE_RPC__WS_URL per eseguire i test live");
    SecretUrl::parse(&raw).expect("SEDETTE_RPC__WS_URL non valido")
}

#[tokio::test]
#[ignore = "richiede SEDETTE_RPC__WS_URL e la rete"]
async fn riceve_nuovi_blocchi_e_il_finalized() {
    let client = ChainClient::connect_ws(&endpoint(), POLYGON_MAINNET_CHAIN_ID)
        .await
        .expect("connessione");

    let mut heads = client.subscribe_heads().await.expect("sottoscrizione");
    let first = tokio::time::timeout(Duration::from_secs(60), heads.next())
        .await
        .expect("nessun blocco entro 60 s")
        .expect("flusso chiuso");
    let second = tokio::time::timeout(Duration::from_secs(60), heads.next())
        .await
        .expect("nessun secondo blocco entro 60 s")
        .expect("flusso chiuso");
    assert!(second.number > first.number);

    let finalized = client
        .finalized()
        .await
        .expect("eth_getBlockByNumber(finalized)")
        .expect("il nodo non fornisce il blocco finalized");
    let latest = client.latest_number().await.expect("eth_blockNumber");
    assert!(finalized.number <= latest);
}

#[tokio::test]
#[ignore = "richiede SEDETTE_RPC__WS_URL e la rete"]
async fn rifiuta_una_chain_diversa() {
    let err = ChainClient::connect_ws(&endpoint(), 1)
        .await
        .expect_err("la chain 137 non deve passare come 1");
    assert!(err.is_fatal(), "{err}");
}
