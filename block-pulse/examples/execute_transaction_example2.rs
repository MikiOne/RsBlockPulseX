use std::convert::TryFrom;
use std::sync::Arc;

use ethers::prelude::*;
use ethers::utils::{hex, keccak256};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 设置以太坊提供者
    let provider = Provider::<Http>::try_from("https://your.ethereum.node")?;

    // 设置钱包
    let wallet: LocalWallet = "YOUR_PRIVATE_KEY".parse()?;
    let client = Arc::new(SignerMiddleware::new(provider, wallet));

    // 合约地址和 ABI
    let contract_address: Address = "YOUR_CONTRACT_ADDRESS".parse()?;
    let abi = include_bytes!("path/to/your/contract_abi.json"); // ABI 文件路径

    // 创建合约实例
    let contract = Contract::from_json(client.clone(), contract_address, abi)?;

    // 准备交易参数
    let to: Address = "RECIPIENT_ADDRESS".parse()?;
    let value: U256 = U256::from(1000000000000000000u64); // 1 ETH
    let data: Vec<u8> = Vec::new(); // 交易数据
    let signatures: Vec<Vec<u8>> = vec![
        hex::decode("SIGNATURE_1").unwrap(),
        hex::decode("SIGNATURE_2").unwrap(),
        // 添加更多签名
    ];

    // 计算 nonce（假设你有一个 nonce 变量）
    let nonce = 0; // 这里需要根据你的逻辑获取 nonce

    // 计算交易哈希
    let tx_hash = get_transaction_hash(&to, value, &data, nonce);

    // 调用 executeTransaction 方法
    let tx = contract.method::<_, H256>("executeTransaction", (to, value, data, signatures))?;

    // 发送交易
    let pending_tx = tx.send().await?;
    println!("Transaction sent: {:?}", pending_tx);

    Ok(())
}

// 计算交易哈希
fn get_transaction_hash(to: &Address, value: U256, data: &[u8], nonce: u64) -> H256 {
    let hash = keccak256(&[
        to.as_bytes(),
        &value.to_fixed_bytes(),
        data,
        &nonce.to_le_bytes(),
    ].concat());
    H256::from_slice(&hash)
}