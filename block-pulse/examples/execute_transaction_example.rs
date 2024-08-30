// src/execute_transaction_example2.rs
use ethers::prelude::*;
use ethers::signers::{LocalWallet, Signer};
use ethers::utils::keccak256;
use std::sync::Arc;
use std::convert::TryFrom;
use ethers::abi::{Abi, Token};
use ethers::core::rand::thread_rng;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 设置提供商和钱包
    let provider = Provider::<Http>::try_from("http://localhost:8545")?;
    // let wallet: LocalWallet = "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee".parse()?;
    let wallet = LocalWallet::new(&mut thread_rng());
    let client = SignerMiddleware::new(provider, wallet);
    let client = Arc::new(client);

    // 合约地址和ABI
    let contract_address: Address = "0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee".parse()?;
    let abi_bytes = include_bytes!("../abis/MultisigWallet.abi");
    let abi: Abi = serde_json::from_slice(abi_bytes)?;

    // let abi: Abi = serde_json::from_str(r#"[{"inputs":[{"internalType":"string","name":"value","type":"string"}],"stateMutability":"nonpayable","type":"constructor"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"address","name":"author","type":"address"},{"indexed":true,"internalType":"address","name":"oldAuthor","type":"address"},{"indexed":false,"internalType":"string","name":"oldValue","type":"string"},{"indexed":false,"internalType":"string","name":"newValue","type":"string"}],"name":"ValueChanged","type":"event"},{"inputs":[],"name":"getValue","outputs":[{"internalType":"string","name":"","type":"string"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"lastSender","outputs":[{"internalType":"address","name":"","type":"address"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"string","name":"value","type":"string"}],"name":"setValue","outputs":[],"stateMutability":"nonpayable","type":"function"}]"#)?;

    // 创建合约实例
    let contract = Contract::new(contract_address, abi, client.clone());

    // 交易参数
    let to: Address = "0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee".parse()?;
    let value: U256 = U256::from(1000); // 发送的以太币数量
    // let data: Bytes = Bytes::from(b"ff"); // 交易数据
    let data: Bytes = Bytes::new(); // 交易数据
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
    let signature = client.sign(tx_hash, &client.address()).await?;

    // 调用合约方法
    // let tx = contract.method::<_, H256>("executeTransaction", (to, value, data, vec![signature]))?
    let ss: [u8; 65] = signature.into();
    let args = vec![Token::Address(to), Token::Uint(value), Token::FixedBytes(data.to_vec()), Token::Bytes(data.to_vec())];
    let exec_tx = contract.method::<_, H256>("executeTransaction", args).unwrap();
    let tx = exec_tx.send().await.unwrap();

    println!("Transaction hash: {:?}", tx);

    Ok(())
}