use log::{info, warn, debug, error, trace};
use std::path::PathBuf;
use structopt::{clap::ArgGroup, StructOpt};
use structopt::clap::AppSettings;
use web3;
use web3::types::U256;
use web3::types::Bytes;
use ethereum_tx_sign::RawTransaction;

use crate::utils;
use crate::ethereum;

const CHAIN_ID: i32 = 5777;

/// 只接受 true 和 false
fn true_or_false(s: &str) -> Result<bool, &'static str> {
    match s {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err("expected `true` or `false`"),
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "ether", global_settings = & [AppSettings::DisableVersion], after_help = "用于以太坊的交易发送")]
pub struct Opt {
    /// 以太坊 RPC 接口地址
    #[structopt(short = "s", long, default_value = "http://localhost:8545")]
    host: String,
    /// gasLimit
    #[structopt(short = "l", long, default_value = "200000")]
    gaslimit: usize,
    /// gasPrice
    #[structopt(short = "p", long, default_value = "200000")]
    gasprice: usize,
    /// 测试的flag
    #[structopt(short = "t", long, default_value = "true", parse(try_from_str = true_or_false), help = "只接收 true 和 false")]
    tf: bool,

    #[structopt(subcommand)]
    subcmd: Option<SubCmd>,

    // #[structopt(short)]
    // verbose: bool,
}

/// Some subcommands
#[derive(StructOpt, Debug, PartialEq)]
pub enum SubCmd {
    /// 上链
    Call(Func),

    /// 查询协约数据
    Query(Func),

    /// 发送ETH交易
    Send(SendEth),

    /// 监控Event
    Listen(Func),

    /// 查询账号的余额
    Balance(Balance),
}

/// The options for C
#[derive(StructOpt, Debug, PartialEq)]
pub struct Func {
    /// 协约的address
    #[structopt(short, long)]
    contract: String,
    /// 函数的名称
    #[structopt(short, long)]
    func: String,
    /// 函数的参数
    #[structopt(short, long)]
    args: Vec<String>,
}

#[derive(StructOpt, Debug, PartialEq)]
pub struct SendEth {
    /// 发送者的私钥
    #[structopt(short, long, required = true, help = "发送者的私钥")]
    from: String,
    /// 接收者的address
    #[structopt(short, long, required = true)]
    to: String,
    /// ETH 数量
    #[structopt(short, long, required = true)]
    value: String,
    /// 交易的附带内容
    #[structopt(short, long, required = false, default_value = "")]
    data: String,
}

#[derive(StructOpt, Debug, PartialEq)]
pub struct Balance {
    /// 以太坊的address
    #[structopt(short, long, required = true)]
    account: String,
}

///
pub async fn get_opt() -> Opt {
    let opt = Opt::from_args();

    match &opt.subcmd {
        Some(SubCmd::Call(Func { contract, func, args })) => print!("{},{},{:?}", contract, func, args),
        Some(SubCmd::Query(Func { contract, func, args })) => print!("{},{},{:?}", contract, func, args),
        Some(SubCmd::Listen(Func { contract, func, args })) => {
            debug!("{},{},{:?}", contract, func, args);
        }
        Some(SubCmd::Balance(balance)) => {
            let bal = balance.balance_of(&opt.host).await;
            debug!("{}", bal.unwrap().to_string());
        }
        Some(SubCmd::Send(send)) => {
            send.send(&opt.host, opt.gaslimit, opt.gasprice).await;
        }
        _ => println!("请填写正确的子命令!")
    }

    return opt;
}

trait Exec {
    fn exec(&self);
}

impl Func {
    /// 执行调用协约的函数
    fn exec(&self, func: &Func) {}
}

impl SendEth {
    /// 发送ETH
    async fn send(&self, host_url: &String, gas_limit: usize, gas_price: usize) {
        // 获得私钥和address对应的H160
        let from_h160 = ethereum::private_to_h160(&self.from);
        let to_h160 = ethereum::address_to_h160(&self.to);

        let web3 = ethereum::get_web3_http(host_url);
        let nonce = web3.eth().transaction_count(from_h160, None).await.unwrap();
        let tx = RawTransaction {
            nonce: utils::convert_u256(nonce),
            to: Some(to_h160.clone()),
            value: ethereum_types::U256::from_dec_str(&self.value).unwrap(),
            gas_price: ethereum_types::U256::from_dec_str(&gas_price.to_string()).unwrap(),
            gas: ethereum_types::U256::from_dec_str(&gas_limit.to_string()).unwrap(),
            data: Vec::new(),
        };

        let signed_tx = tx.sign(&utils::get_private_key(&self.from), &CHAIN_ID);

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
}

impl Balance {
    async fn balance_of(&self, host_url: &String) -> std::result::Result<U256, web3::Error> {
        let web3 = ethereum::get_web3_http(host_url);
        let account = utils::address_to_H160(&self.account);
        let balance = web3.eth().balance(account, None).await?;
        debug!("Balance of {:?}: {}", account, balance);
        return Ok(balance);
    }
}