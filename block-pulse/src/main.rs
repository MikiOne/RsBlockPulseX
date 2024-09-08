use ethers::prelude::*;
use std::convert::TryFrom;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // let provider = Provider::<Http>::try_from("http://192.168.101.45:8545")?;
    let provider = Provider::<Http>::try_from("https://rpc-sepolia.flashbots.net/")?;

    // 指定要查询的地址
    // 注意这是是合约地址，主币转账没有Log
    let address1: H160 = "0x719dAE308177a5AABC6c68E51580A771C4dB0F47".parse()?;
    let address2: H160 = "0x837124F129BB89DAc665fBa041A2470e9e79FE32".parse()?;
    let addresses = vec![address1, address2];

    // 设置过滤器
    let filter = Filter::new()
        .address(addresses)
        .from_block(BlockNumber::Number(6624060.into()))
        .to_block(BlockNumber::Latest);

    // 获取日志
    let logs = provider.get_logs(&filter).await?;

    // Log: Log { address: 0x837124f129bb89dac665fba041a2470e9e79fe32, topics: [0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef, 0x0000000000000000000000000000000000000000000000000000000000000000, 0x000000000000000000000000ac207d50ec3150a3420709295154bbf8e80bd876], data: Bytes(0x00000000000000000000000000000000000000000000152d02c7e14af6800000), block_hash: Some(0x9fef7555cc78d13d99276ae96ae527421ba6bb8fb9f989217cfe631b7afae46f), block_number: Some(6624063), transaction_hash: Some(0x6d15049bf868cdc12d6e80e1beb270e081447da5cbce8f014c86910b2c477ddf), transaction_index: Some(38), log_index: Some(114), transaction_log_index: None, log_type: None, removed: Some(false) }
    // Log: Log { address: 0x837124f129bb89dac665fba041a2470e9e79fe32, topics: [0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef, 0x000000000000000000000000ac207d50ec3150a3420709295154bbf8e80bd876, 0x0000000000000000000000009db47d9b867231b1313f319dc9380c1168a9a873], data: Bytes(0x00000000000000000000000000000000000000000000021e19e0c9bab2400000), block_hash: Some(0x84d64ec66b9590fdcc5be047534dfb7ba2226482dc60cd5920e4ea2587e5d74c), block_number: Some(6624068), transaction_hash: Some(0x21c382601e128406e6ba01bac1be904721e9231710b2a2aa113f55ee91fb1c0e), transaction_index: Some(17), log_index: Some(38), transaction_log_index: None, log_type: None, removed: Some(false) }
    for log in logs {
        println!("Log: {:?}", log);
    }

    Ok(())
}