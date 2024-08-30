use std::convert::TryFrom;
use std::sync::Arc;

use ethers::abi::Abi;
use ethers::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // this is a fake address used just for this example
    let address = "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee".parse::<Address>()?;

    // (ugly way to write the ABI inline, you can otherwise read it from a file)
    let abi: Abi = serde_json::from_str(
        r#"[{"inputs":[{"internalType":"string","name":"value","type":"string"}],"stateMutability":"nonpayable","type":"constructor"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"address","name":"author","type":"address"},{"indexed":true,"internalType":"address","name":"oldAuthor","type":"address"},{"indexed":false,"internalType":"string","name":"oldValue","type":"string"},{"indexed":false,"internalType":"string","name":"newValue","type":"string"}],"name":"ValueChanged","type":"event"},{"inputs":[],"name":"getValue","outputs":[{"internalType":"string","name":"","type":"string"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"lastSender","outputs":[{"internalType":"address","name":"","type":"address"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"string","name":"value","type":"string"}],"name":"setValue","outputs":[],"stateMutability":"nonpayable","type":"function"}]"#,
    )?;

    // connect to the network
    let client = Provider::<Http>::try_from("http://localhost:8545").unwrap();

    // create the contract object at the address
    let contract = Contract::new(address, abi, Arc::new(client));

    // Calling constant methods is done by calling `call()` on the method builder.
    // (if the function takes no arguments, then you must use `()` as the argument)
    let init_value: String = contract.method::<_, String>("getValue", ())?.call().await?;

    // Non-constant methods are executed via the `send()` call on the method builder.
    let call = contract.method::<_, H256>("setValue", "hi".to_owned())?;
    let pending_tx = call.send().await?;

    // `await`ing on the pending transaction resolves to a transaction receipt
    let receipt = pending_tx.confirmations(6).await?;

    Ok(())
}
