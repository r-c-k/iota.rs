// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example balance --release
use iota::{Client, Seed};
extern crate dotenv;
use dotenv::dotenv;
use std::env;

/// In this example we will get the balance of a known address

#[tokio::main]
async fn main() {
    // Create a client instance
    let iota = Client::builder()
        .with_node("https://api.hornet-0.testnet.chrysalis2.com") // Insert the node here
        .unwrap()
        .with_node_sync_disabled()
        .finish()
        .await
        .unwrap();

    // This example uses dotenv, which is not safe for use in production
    dotenv().ok();

    let seed = Seed::from_bytes(&hex::decode(env::var("NONSECURE_USE_OF_DEVELOPMENT_SEED_1").unwrap()).unwrap());

    let seed_balance = iota.get_balance(&seed).finish().await.unwrap();
    println!("Account balance: {:?}i\n", seed_balance);

    let address = "atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r";

    let response = iota.get_address().balance(&address.into()).await.unwrap();
    println!("The balance of {:?} is {:?}i\n", address, response.balance);

    let outputs = iota
        .get_address()
        .outputs(&address.into(), Default::default())
        .await
        .unwrap();
    println!("The outputs of {:?} are {:?}\n", address, outputs);

    let output = iota.get_output(&outputs[0]).await.unwrap();
    println!("Output {:?}", output);
}
