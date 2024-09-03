use std::sync::Arc;

use ethers::{
    core::types::{Address, Filter, H160, H256, U256},
    providers::{Http, Middleware, Provider},
};
use ethers::prelude::*;
use eyre::Result;

const HTTP_URL: &str = "http://192.168.101.45:8545";
const ERC_ADDRESS: &str = "0xDc64a140Aa3E981100a9becA4E685f962f0cF6C9";
const FROM_ADDRESS: &str = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266";
const USER_ADDRESS: &str = "0x70997970C51812dc3A010C7d01b50e0d17dc79C8";
// const USDT_ADDRESS: &str = "0xdAC17F958D2ee523a2206206994597C13D831ec7";

/// This example demonstrates filtering and parsing event logs by fetching all Uniswap V3 pools
/// where both tokens are in the set [USDC, USDT, DAI].
///
/// V3 factory reference: https://github.com/Uniswap/v3-core/blob/main/contracts/interfaces/IUniswapV3Factory.sol
#[tokio::main]
async fn main() -> Result<()> {
    let provider = Provider::<Http>::try_from(HTTP_URL)?;
    let client = Arc::new(provider);
    let token_topics = [
        H256::from(USER_ADDRESS.parse::<H160>()?),
        // H256::from(USDT_ADDRESS.parse::<H160>()?),
        H256::from(FROM_ADDRESS.parse::<H160>()?),
    ];
    let filter = Filter::new()
        .address(ERC_ADDRESS.parse::<Address>()?)
        // event Transfer(address indexed from, address indexed to, uint256 value);
        // .event("PoolCreated(address,address,uint24,int24,address)")
        .event("Transfer(address,address,uint256)")
        .topic1(token_topics.to_vec())
        .topic2(token_topics.to_vec())
        .from_block(0);
    println!("filter: {:?}", filter);
    let logs = client.get_logs(&filter).await?;
    // logs: [Log { address: 0xdc64a140aa3e981100a9beca4e685f962f0cf6c9,
    // topics: [0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef, 0x000000000000000000000000f39fd6e51aad88f6f4ce6ab8827279cfffb92266, 0x00000000000000000000000070997970c51812dc3a010c7d01b50e0d17dc79c8], data: Bytes(0x0000000000000000000000000000000000000000000000000de0b6b3a7640000),
    // block_hash: Some(0xebce27f398bf5bf551e1391f2a692794511bd2ba61b41e70604dff3249941f9a),
    // block_number: Some(16), transaction_hash: Some(0x8da8f20a3679673723499f3a46dee2e09da4d2a85baad12081e20bb9a663c1ef),
    // transaction_index: Some(0), log_index: Some(0),
    // transaction_log_index: None, log_type: None, removed: Some(false) }]
    println!("logs: {:?}", logs);

    println!("{} pools found!", logs.iter().len());
    for log in logs.iter() {
        let token0 = Address::from(log.topics[1]);
        let token1 = Address::from(log.topics[2]);
        let fee_tier = U256::from_big_endian(&log.topics[3].as_bytes()[29..32]);
        // data: 0x
        // 000000000000000000000000000000000000000000000000000000000000(29)00c8(31)
        // 00000000000000000000000028(44)18ea851dcfaeeb4633fc8aae08b7063d32e4f5(63)
        let tick_spacing = U256::from_big_endian(&log.data[29..32]);
        let pool = Address::from(&log.data[44..64].try_into()?);
        println!(
            "pool = {pool}, token0 = {token0}, token1 = {token1}, fee = {fee_tier}, spacing = {tick_spacing}"
        );
    }
    Ok(())
}