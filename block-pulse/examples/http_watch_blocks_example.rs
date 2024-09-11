use ethers::providers::{PubsubClient, Provider, StreamExt, Http};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = Provider::<dyn PubsubClient>::try_from("http://192.168.101.45:8545/")?;
    let mut stream = provider.subscribe_blocks().await?;

    while let Some(block) = stream.next().await {
        match block {
            Ok(block) => {
                println!("New block: {:?}", block);
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
            }
        }
    }
    Ok(())
}
