#[macro_use]
extern crate log;
extern crate env_logger;
extern crate web3;
extern crate hex;
extern crate ethereum_types;
extern crate ethkey;
extern crate eth_checksum;


mod cmd;
mod utils;
mod ethereum;

use cmd::get_opt;

/**
ether --abi ./abi.json sendEth --from 0xXXXX --to 0xYYY
ether --abi ./abi.json listenEvent --contract 0xXXXX --name funcName # 需要解析topic
ether --abi ./abi.json call --contract 0xXXXX --func funcName --args arg1 arg2 arg3 ...
ether --abi ./abi.json query --contract 0xXXXX --func funcName --args arg1 arg2 arg3 ...
*/

#[tokio::main]
async fn main() {
    env_logger::init();

    let opt = get_opt().await;
    // debug!("{:#?}", opt);

    // ethereum::private_to_address(&String::from("941b9e919770751c4b0561ea39526c087d10925fd9815073059c63f963740f6c"));
}