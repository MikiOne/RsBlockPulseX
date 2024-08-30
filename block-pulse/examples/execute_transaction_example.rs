// src/execute_transaction_example2.rs
use ethers::prelude::*;
use ethers::signers::{LocalWallet, Signer};
use ethers::utils::keccak256;
use std::sync::Arc;
use std::convert::TryFrom;
use ethers::abi::Token;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 设置提供商和钱包
    let provider = Provider::<Http>::try_from("http://localhost:8545")?;
    let wallet: LocalWallet = "YOUR_PRIVATE_KEY".parse()?;
    let client = SignerMiddleware::new(provider, wallet);
    let client = Arc::new(client);

    // 合约地址和ABI
    let contract_address: Address = "YOUR_CONTRACT_ADDRESS".parse()?;
    let abi = include_bytes!("../abis/MultisigWallet.abi");

    // 创建合约实例
    let contract = Contract::from_json(client.clone(), contract_address, abi)?;

    // 交易参数
    let to: Address = "RECIPIENT_ADDRESS".parse()?;
    let value: U256 = U256::from(1000); // 发送的以太币数量
    let data: Bytes = Bytes::from("0x"); // 交易数据
    let nonce: U256 = U256::from(0); // 交易的nonce

    // 获取交易哈希
    let tx_hash = keccak256(abi::encode(&[
        Token::Address(contract_address),
        Token::Address(to),
        Token::Uint(value),
        Token::Bytes(data.clone().to_vec()),
        Token::Uint(nonce),
    ]));

    // 签名交易哈希
    let signature = client.sign_hash(tx_hash, None).await?;

    // 调用合约方法
    let tx = contract.method::<_, H256>("executeTransaction", (to, value, data, vec![signature]))?
        .send()
        .await?;

    println!("Transaction hash: {:?}", tx);

    Ok(())
}