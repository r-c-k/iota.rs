// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example custom_inputs --release
use iota::{Client, Seed};
extern crate dotenv;
use dotenv::dotenv;
use std::env;

/// In this example, we send 1_000_000 tokens to atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r
/// This address belongs to the first seed in .env.example

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

    // First address from the seed below is atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r
    let seed = Seed::from_bytes(&hex::decode(env::var("NONSECURE_USE_OF_DEVELOPMENT_SEED_1").unwrap()).unwrap());

    let address = iota.get_addresses(&seed).with_range(0..1).finish().await.unwrap();
    println!("{:?}", address[0]);

    let outputs = iota
        .get_address()
        .outputs(&address[0], Default::default())
        .await
        .unwrap();
    println!("{:?}", outputs);

    let message = iota
        .message()
        .with_seed(&seed)
        .with_input(outputs[0].clone())
        //.with_input_range(20..25)
        .with_output(
            &"atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r".into(),
            1_000_000,
        )
        .unwrap()
        .finish()
        .await
        .unwrap();

    println!(
        "Transaction sent: https://explorer.iota.org/chrysalis/message/{}",
        message.id().0
    );
}
