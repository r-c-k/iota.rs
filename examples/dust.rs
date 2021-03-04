// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example dust --release
use iota::{Client, Seed};
extern crate dotenv;
use dotenv::dotenv;
use std::env;

/// In this example we send a dust allowance output and dust

#[tokio::main]
async fn main() {
    // Create a client instance
    let iota = Client::builder()
        .with_node("https://api.hornet-0.testnet.chrysalis2.com") // Insert the node here
        .unwrap()
        .finish()
        .await
        .unwrap();

    // This example uses dotenv, which is not safe for use in production
    dotenv().ok();

    let seed = Seed::from_bytes(&hex::decode(env::var("NONSECURE_USE_OF_DEVELOPMENT_SEED_1").unwrap()).unwrap());

    let message = iota
        .message()
        .with_seed(&seed)
        .with_dust_allowance_output(
            &"atoi1qpnrumvaex24dy0duulp4q07lpa00w20ze6jfd0xly422kdcjxzakzsz5kf".into(),
            1_000_000,
        )
        .unwrap()
        .with_output(
            &"atoi1qpnrumvaex24dy0duulp4q07lpa00w20ze6jfd0xly422kdcjxzakzsz5kf".into(),
            1,
        )
        .unwrap()
        .finish()
        .await
        .unwrap();

    println!(
        "First transaction sent: https://explorer.iota.org/chrysalis/message/{}",
        message.id().0
    );
}
