use log::{info, warn, debug, error, trace};
use std::path::PathBuf;
use structopt::{clap::ArgGroup, StructOpt};
use structopt::clap::AppSettings;
use web3;
use web3::types::U256;
use web3::types::Bytes;
use ethereum_tx_sign::RawTransaction;
use ether_lib;

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
        ethereum::send_eth(host_url, &self.from, &self.to, gas_limit, gas_price, &self.value).await;
    }
}

impl Balance {
    async fn balance_of(&self, host_url: &String) -> std::result::Result<U256, web3::Error> {
        return ethereum::balance_of(host_url, &self.account).await;
        // let web3 = ethereum::get_web3_http(host_url);
        // // let account = utils::address_to_H160(&self.account);
        // let account = ether_lib::address_to_h160(&self.account);
        // let balance = web3.eth().balance(account, None).await?;
        // debug!("Balance of {:?}: {}", account, balance);
        // return Ok(balance);
    }
}