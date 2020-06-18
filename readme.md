
# 发送交易
```
ether -h
ether 0.1.0
Some subcommands

USAGE:
    ether [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help    Prints help information

OPTIONS:
    -l, --gaslimit <gaslimit>    gasLimit [default: 200000]
    -p, --gasprice <gasprice>    gasPrice [default: 200000]
    -s, --host <host>            以太坊 RPC 接口地址 [default: http://localhost:8545]
    -t, --tf <tf>                只接收 true 和 false [default: true]

SUBCOMMANDS:
    balance    查询账号的余额
    call       上链
    help       Prints this message or the help of the given subcommand(s)
    listen     监控Event
    query      查询协约数据
    send       发送ETH交易

用于以太坊的交易发送

```

```
# 发送 ETH
ether --abi ./abi.json sendEth --from 0xXXXX --to 0xYYY
# 监听 EVENT 
ether --abi ./abi.json listenEvent --contract 0xXXXX --name funcName # 需要解析topic
# 调用协约的方法上链
ether --abi ./abi.json call --contract 0xXXXX --func funcName --args arg1 arg2 arg3 ...
# 查询协约中的数据
ether --abi ./abi.json query --contract 0xXXXX --func funcName --args arg1 arg2 arg3 ...
```
