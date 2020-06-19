extern crate ethkey;
extern crate eth_checksum;
extern crate hex;
extern crate log;
extern crate env_logger;

use log::debug;

/// 剔除 hex 的 0x前缀
fn trim_0x(str_hex: &String) -> &str {
    let addr_no_0x;
    if str_hex.starts_with("0x") {
        addr_no_0x = &str_hex[2..];
    } else {
        addr_no_0x = &str_hex[..];
    }
    return addr_no_0x;
}

/// 私钥 转 [u8;20] address
pub fn private_to_u8_address(private: &String) -> [u8; 20] {
    let sect = hex::decode(trim_0x(private)).unwrap();
    let secret_key = ethkey::SecretKey::from_raw(&sect).unwrap();
    let pub_key = secret_key.public();
    let pub_key_u8 = pub_key.address().clone();
    return pub_key_u8;
}

/// 私钥转address string
pub fn private_to_address(private: &String) -> String {
    let pub_key_u8 = private_to_u8_address(private);
    let pub_key_string = hex::encode(&pub_key_u8);
    let pub_key_string_checksummed = eth_checksum::checksum(&pub_key_string);
    debug!("private_Key:{}", private);
    debug!("pub_key_string_checksummed:{}", pub_key_string_checksummed);
    return pub_key_string_checksummed;
}

/// 私钥 转 web3::types::H160
pub fn private_to_h160(private: &String) -> web3::types::H160 {
    let pub_key_u8 = private_to_u8_address(private);
    let address_u160 = web3::types::H160::from_slice(&pub_key_u8);
    return address_u160;
}

/// address 转 web3::types::H160
pub fn address_to_h160(address: &String) -> web3::types::H160 {
    let addr_u = hex::decode(trim_0x(address)).unwrap();
    // let ux = utils::to_array20(&u);
    let address_u160 = web3::types::H160::from_slice(&addr_u);
    return address_u160;
}

/// 私钥 转 web3::types::H256
pub fn private_to_web3_h256(private_hex: &String) -> web3::types::H256 {
    let private_key = hex::decode(trim_0x(private_hex)).unwrap();
    web3::types::H256(to_array32(private_key.as_slice()))
}

/// 私钥 转 ethereum_types::H256
pub fn private_to_ethereum_types_h256(private_hex: &String) -> ethereum_types::H256 {
    let private_key = hex::decode(trim_0x(private_hex)).unwrap();
    ethereum_types::H256(to_array32(private_key.as_slice()))
}

/// 获得固定长度的数组
pub fn to_array32(bytes: &[u8]) -> [u8; 32] {
    let mut array = [0; 32];
    let bytes = &bytes[..array.len()];
    array.copy_from_slice(bytes);
    array
}

pub fn to_array20(bytes: &[u8]) -> [u8; 20] {
    let mut array = [0; 20];
    let bytes = &bytes[..array.len()];
    array.copy_from_slice(bytes);
    array
}

/// 根据私钥hex转换为ethereum_types的H256
pub fn get_private_key(private_hex: &String) -> ethereum_types::H256 {
    let private_key = hex::decode(trim_0x(private_hex)).unwrap();
    ethereum_types::H256(to_array32(private_key.as_slice()))
}

/// 将web3的 U256 转换为 ethereum_types的U256
pub fn web3_to_ethereum_types_u256(value: web3::types::U256) -> ethereum_types::U256 {
    return ethereum_types::U256(value.0);
}

/// 将web3的 H160（Address）转换为 ethereum_types的H160
pub fn web3_to_ethereum_types_h160(value: web3::types::H160) -> ethereum_types::H160 {
    return ethereum_types::H160(value.0);
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_private_to_address() {
        let private = "941b9e919770751c4b0561ea39526c087d10925fd9815073059c63f963740f6c";
        let private2 = "0x941b9e919770751c4b0561ea39526c087d10925fd9815073059c63f963740f6c";
        let address = "0x19f69286B498e9dEA3bBB266B83D2B9f3B9f482C";
        let addr = crate::private_to_address(&String::from(private));
        let addr2 = crate::private_to_address(&String::from(private2));
        assert_eq!(address, addr);
        assert_eq!(address, addr2);
    }
}
