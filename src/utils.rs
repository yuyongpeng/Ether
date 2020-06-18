use web3::types::{H160, H256};
use ethereum_types;
use web3;
use hex;

/// 将address的hex转为web使用的H160
pub fn address_to_H160(addr: &String) -> H160 {
    let addr_no_hex;
    if addr.starts_with("0x"){
        addr_no_hex = &addr[2..];
    }else{
        addr_no_hex = &addr[..];
    }
    let de = hex::decode(addr_no_hex).unwrap();
    let x = de.as_slice();
    return H160::from_slice(x);
}

/// 私钥转address
pub fn private_to_address(private: &String){

}

/// 获得固定长度的数组
pub fn to_array32(bytes: &[u8]) -> [u8;32]{
    let mut array = [0;32];
    let bytes = &bytes[..array.len()];
    array.copy_from_slice(bytes);
    array
}
pub fn to_array20(bytes: &[u8]) -> [u8;20]{
    let mut array = [0;20];
    let bytes = &bytes[..array.len()];
    array.copy_from_slice(bytes);
    array
}

/// 根据私钥hex转换为ethereum_types的H256
pub fn get_private_key(private_hex: &String) -> ethereum_types::H256{
    let private_no_hex;
   if private_hex.starts_with("0x"){
       private_no_hex = &private_hex[2..];
   }else{
       private_no_hex = &private_hex[..];
    }
    let private_key = hex::decode(private_no_hex).unwrap();
    ethereum_types::H256(to_array32(private_key.as_slice()))
}

/// 将web3的 U256 转换为 ethereum_types的U256
pub fn convert_u256(value: web3::types::U256)-> ethereum_types::U256{
    return ethereum_types::U256(value.0);
}
/// 将web3的 H160（Address）转换为 ethereum_types的H160
pub fn convert_account(value: web3::types::H160) -> ethereum_types::H160{
    let ret = ethereum_types::H160::from(value.0);
    return ret;
}











