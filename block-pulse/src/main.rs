use ethers::prelude::*;
use std::convert::TryFrom;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let provider = Provider::<Http>::try_from("http://192.168.101.45:8545")?;

    // 指定要查询的地址
    let address: H160 = "0xDc64a140Aa3E981100a9becA4E685f962f0cF6C9".parse()?;

    // 设置过滤器
    let filter = Filter::new()
        .address(address)
        .from_block(BlockNumber::Number(0.into()))
        .to_block(BlockNumber::Latest);

    // 获取日志
    let logs = provider.get_logs(&filter).await?;

    for log in logs {
        println!("Log: {:?}", log);
    }

    Ok(())
}