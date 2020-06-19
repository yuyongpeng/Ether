// #[macro_use]
// extern crate tokio;
use log::debug;
use std::time;
use web3::contract::{Contract, Options};
use web3::futures::StreamExt;
use web3::types::FilterBuilder;
use web3::types::{H160, H256, U256, Bytes};
use tokio::prelude::*;
use std::process;
// use bytes::{Bytes, BytesMut, Buf, BufMut};
use ethkey;
use eth_checksum;
use ethereum_tx_sign::RawTransaction;
// use tokio_test;

use crate::utils;

const CHAIN_ID: i32 = 5777;

#[tokio::main]
async fn main2() -> web3::contract::Result<()> {
    let _ = env_logger::try_init();
    let web3 = web3::Web3::new(web3::transports::Http::new("http://106.75.96.223:38545")?);

    // Get the contract bytecode for instance from Solidity compiler
    let bytecode = include_str!("/Users/yuyongpeng/CLionProjects/macros/res/SimpleEvent.bin");

    let accounts = web3.eth().accounts().await?;
    println!("accounts: {:?}", &accounts);

    let contract = Contract::deploy(web3.eth(), include_bytes!("/Users/yuyongpeng/CLionProjects/macros/res/SimpleEvent.abi"))?
        .confirmations(1)
        .poll_interval(time::Duration::from_secs(10))
        .options(Options::with(|opt| opt.gas = Some(3_000_000.into())))
        .execute(bytecode, (), accounts[8])?
        .await?;

    println!("contract deployed at: {}", contract.address());

    // Filter for Hello event in our contract
    let filter = FilterBuilder::default()
        .address(vec![contract.address()])
        // .topics(
        //     Some(vec!["d282f389399565f3671145f5916e51652b60eee8e5c759293a2f5771b8ddfd2e"
        //         .parse()
        //         .unwrap()]),
        //     None,
        //     None,
        //     None,
        // )
        .build();

    let filter = web3.eth_filter().create_logs_filter(filter).await?;

    let mut logs_stream = filter.stream(time::Duration::from_secs(1));

    let tx = contract.call("hello", (), accounts[8], Options::default()).await?;
    println!("got tx: {:?}", tx);

    let log = logs_stream.next().await.unwrap();
    let logx = log.unwrap();
    println!("got log: {:?}", &logx);
    let h256 = H256::from_slice(&logx.data.0);
    let addr = H160::from(h256);
    println!("{:?}", addr);
    Ok(())
}

/// 获得 http的 web3 对象
pub fn get_web3_http(url: &String) -> web3::Web3<web3::transports::Http> {
    // let transport = T::new(url).unwrap();
    let transport = web3::transports::Http::new(url).unwrap();
    let web3 = web3::Web3::new(transport);
    return web3;
}

/// 获得 websocket的 web3 对象
pub async fn get_web3_websocket(url: &String) -> web3::Web3<web3::transports::WebSocket> {
    // let transport = T::new(url).unwrap();
    let transport = web3::transports::WebSocket::new(url).await.unwrap();
    let web3 = web3::Web3::new(transport);
    return web3;
}

/// 发送 ETH
pub async fn send_eth(host_url: &String, from: &String, to: &String, gas_limit: usize, gas_price: usize, value: &String) {
    // 获得私钥和address对应的H160
    let from_h160 = ether_lib::private_to_h160(from);
    let to_h160 = ether_lib::address_to_h160(to);
    // let from_h160 = ethereum::private_to_h160(&self.from);
    // let to_h160 = ethereum::address_to_h160(&self.to);

    let web3 = get_web3_http(host_url);
    let nonce = web3.eth().transaction_count(from_h160, None).await.unwrap();
    let tx = RawTransaction {
        nonce: ether_lib::web3_to_ethereum_types_u256(nonce),
        to: Some(to_h160.clone()),
        value: ethereum_types::U256::from_dec_str(value).unwrap(),
        gas_price: ethereum_types::U256::from_dec_str(&gas_price.to_string()).unwrap(),
        gas: ethereum_types::U256::from_dec_str(&gas_limit.to_string()).unwrap(),
        data: Vec::new(),
    };
    let signed_tx = tx.sign(&ether_lib::private_to_ethereum_types_h256(from), &CHAIN_ID);

    let from_balance_before = web3.eth().balance(from_h160.clone(), None).await.unwrap();
    let to_balance_before = web3.eth().balance(to_h160.clone(), None).await.unwrap();
    let tx_hash = web3.eth().send_raw_transaction(Bytes::from(signed_tx)).await.unwrap();
    let from_balance_after = web3.eth().balance(from_h160.clone(), None).await.unwrap();
    let to_balance_after = web3.eth().balance(to_h160.clone(), None).await.unwrap();

    debug!("send ETH TX Hash: {:?}", tx_hash);
    println!("from Balance before: {}", from_balance_before);
    println!("to Balance before: {}", to_balance_before);
    println!("from Balance after: {}", from_balance_after);
    println!("to Balance after: {}", to_balance_after);
}

/// 查询账号的余额
pub async fn balance_of(host_url: &String, account_address: &String) -> std::result::Result<U256, web3::Error> {
    let web3 = get_web3_http(host_url);
    // let account = utils::address_to_H160(&self.account);
    let account = ether_lib::address_to_h160(account_address);
    let balance = web3.eth().balance(account, None).await?;
    debug!("Balance of {:?}: {}", account, balance);
    return Ok(balance);
}

#[cfg(test)]
mod tests {
    const url: &str = "http://localhost:8545";
    const from_private: &str = "941b9e919770751c4b0561ea39526c087d10925fd9815073059c63f963740f6c";
    const to_account: &str = "0x3Cf914e5cfFbe5AA975571701f2264FECC438533";

    #[test]
    fn test_send_eth() {
        // let url = "http://localhost:8545";
        // let from_private = "941b9e919770751c4b0561ea39526c087d10925fd9815073059c63f963740f6c";
        // let to_account = "0x3Cf914e5cfFbe5AA975571701f2264FECC438533";
        let gas_limit = 200000;
        let gas_price = 200000;
        let value = "2";
        let value_u256 = web3::types::U256::from_dec_str(value).unwrap();
        let balance = tokio_test::block_on(crate::ethereum::balance_of(&String::from(url), &String::from(to_account))).unwrap();
        println!("{}", balance);
        tokio_test::block_on(crate::ethereum::send_eth(&String::from(url), &String::from(from_private),
                                                       &String::from(to_account), gas_limit, gas_price,
                                                       &String::from(value)));
        let balance_new = tokio_test::block_on(crate::ethereum::balance_of(&String::from(url), &String::from(to_account))).unwrap();

        assert_eq!(balance, balance_new.checked_sub(value_u256).unwrap())
    }

    #[test]
    fn test_balance_of() {}
}