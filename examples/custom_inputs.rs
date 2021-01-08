// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example custom_inputs --release
use iota::{Client, Seed};
use std::num::NonZeroU64;
/// In this example, we send 100 tokens to iot1q86rlrygq5wcgdwt7fpajaxxppc49tg0jk0xadnp66fsfjtwt8vgc48sse6
/// This address belongs to the seed "256a818b2aac458941f7274985a410e57fb750f3a3a67369ece5bd9ae7eef5b0"

#[tokio::main]
async fn main() {
    let iota = Client::build() // Crate a client instance builder
        .with_node("https://api.lb-0.testnet.chrysalis2.com") // Insert the node here
        .unwrap()
        .finish()
        .unwrap();

    // Insert your seed. Since the output amount cannot be zero. The seed must contain non-zero balance.
    // First address from the seed below is iot1qxt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupxgecea4
    let seed = Seed::from_ed25519_bytes(
        &hex::decode("256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b2").unwrap(),
    )
    .unwrap();

    let address = iota.find_addresses(&seed).account_index(0).range(0..1).get().unwrap();
    println!("{:?}", address[0].0.to_bech32());
    let outputs = iota.get_address().outputs(&address[0].0).await.unwrap();
    println!("{:?}", outputs);

    let message_id = iota
        .send()
        .transaction(&seed)
        .with_input(outputs[1].clone())
        .with_output(
            "iot1q86rlrygq5wcgdwt7fpajaxxppc49tg0jk0xadnp66fsfjtwt8vgc48sse6",
            NonZeroU64::new(100).unwrap(),
        )
        .unwrap()
        .finish()
        .await
        .unwrap();

    println!(
        "Transaction sent: https://explorer.iota.org/chrysalis/message/{}",
        message_id
    );
}