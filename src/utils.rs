use std::str::FromStr;
use substreams::scalar::BigInt;

use crate::pb::dcl;

pub fn get_item_id(contract_address: String, item_id: String) -> std::string::String {
    let mut id = contract_address;
    id.push_str(&String::from("-"));
    id.push_str(&item_id);
    id
}

impl From<substreams::scalar::BigInt> for dcl::BigInt {
    fn from(n: substreams::scalar::BigInt) -> Self {
        dcl::BigInt {
            value: n.to_string(),
        }
    }
}

impl From<dcl::BigInt> for BigInt {
    fn from(n: dcl::BigInt) -> Self {
        BigInt::from_str(&n.value).unwrap()
    }
}

pub fn sanitize_sql_string(mut str: String) -> String {
    let cloned = str.clone();
    let v: Vec<_> = cloned.match_indices("'").collect();
    let mut quotes_scaped = 0;
    for idx in v {
        str.insert(idx.0 + quotes_scaped, '\'');
        quotes_scaped = quotes_scaped + 1;
    }
    str
}
