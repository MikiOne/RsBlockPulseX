use ethers::providers::{Middleware, Provider, StreamExt, Ws};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let provider =
        Provider::<Ws>::connect("wss://mainnet.infura.io/ws/v3/c60b0bb42f8a4c6481ecd229eddaca27")
            .await?;
    let mut stream = provider.subscribe_blocks().await?.take(2);
    while let Some(block) = stream.next().await {
        println!(
            "Ts: {:?}, block number: {} -> {:?}",
            block.timestamp,
            block.number.unwrap(),
            block.hash.unwrap()
        );
        println!("block: {:?}", block);
    }

    Ok(())
}

// use ethers::prelude::{Http, Provider};
//
//
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let provider = Provider::<Http>::try_from("http://192.168.101.45:8545/")?;
//     let mut stream = provider.subscribe_blocks().await?;
//
//     while let Some(block) = stream.next().await {
//         match block {
//             Ok(block) => {
//                 println!("New block: {:?}", block);
//             }
//             Err(e) => {
//                 eprintln!("Error: {:?}", e);
//             }
//         }
//     }
//     Ok(())
// }
