use ethers::prelude::*;
use std::convert::TryFrom;

// use ethers::prelude::{Http, Provider};
use ethers::providers::{Http, Provider, StreamExt, Ws};
use eyre::Result;
use ethers_middleware::Middleware;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = Provider::<Http>::try_from("http://192.168.101.45:8545")?;

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

// #[tokio::main]
// async fn main() -> eyre::Result<()> {
//     // 设置 WebSocket 提供者
//     let ws_provider = Provider::<Ws>::try_from("wss://192.168.101.45:8545")?;
//
//     // 创建一个流来监听新的区块
//     let mut stream = ws_provider.watch_blocks().await?;
//
//     while let Some(block) = stream.next().await {
//         println!("New block: {:?}", block);
//
//         // 获取区块中的所有交易
//         let block = ws_provider.get_block_with_txs(block.number).await?;
//         if let Some(block) = block {
//             for tx in block.transactions {
//                 println!("Transaction: {:?}", tx);
//             }
//         }
//     }
//
//     Ok(())
// }