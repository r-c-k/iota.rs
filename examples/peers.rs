// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example peers --release
use iota::Client;

/// In this example we get information about the nodes peers

#[tokio::main]
async fn main() {
    // Create a client instance
    let iota = Client::builder()
        .with_node("https://api.hornet-0.testnet.chrysalis2.com") // Insert the node here
        .unwrap()
        .finish()
        .await
        .unwrap();

    let peers = iota.get_peers().await.unwrap();
    println!("Peers: {:?}", peers);
}
