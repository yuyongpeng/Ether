// #[macro_use]
// extern crate tokio;
use log::debug;
use std::time;
use web3;
use web3::contract::{Contract, Options};
use web3::futures::StreamExt;
use web3::types::FilterBuilder;
use web3::types::H160;
use web3::types::H256;
use tokio::prelude::*;
use std::process;
use hex;
use bytes;
use bytes::{Bytes, BytesMut, Buf, BufMut};
use ethkey;
use eth_checksum;

use crate::utils;


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

/// 私钥转address string
pub fn private_to_address(private: &String) -> String {
    let sect = hex::decode(private).unwrap();
    let secret_key = ethkey::SecretKey::from_raw(&sect).unwrap();
    let pub_key = secret_key.public();
    let pub_key_u8 = pub_key.address();
    // let add = ethkey::Address::from(*pub_key_u8);
    let pub_key_string = hex::encode(pub_key_u8);
    let pub_key_string_checksummed = eth_checksum::checksum(&pub_key_string);
    debug!("private_Key:{}", private);
    debug!("pub_key_string_checksummed:{}", pub_key_string_checksummed);
    return pub_key_string_checksummed;
}

/// 私钥转H160
pub fn private_to_h160(private: &String) -> web3::types::H160 {
    let sect = hex::decode(private).unwrap();
    let secret_key = ethkey::SecretKey::from_raw(&sect).unwrap();
    let pub_key = secret_key.public();
    let pub_key_u8 = pub_key.address();
    let address_u160 = web3::types::H160::from_slice(pub_key_u8);
    return address_u160;
}

///
pub fn address_to_h160(address: &String) -> web3::types::H160 {
    let address_no_0x;
    if address.starts_with("0x") {
        address_no_0x = &address[2..];
    } else {
        address_no_0x = &address[..];
    }
    let addr_u = hex::decode(address_no_0x).unwrap();
    // let ux = utils::to_array20(&u);
    let address_u160 = web3::types::H160::from_slice(&addr_u);
    return address_u160;
}