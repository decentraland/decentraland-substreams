use crate::abi;
use substreams::{scalar::BigInt, Hex};
use substreams_ethereum::rpc::RpcBatch;

pub fn collection_data_call(collection_address: Vec<u8>) -> String {
    let responses = RpcBatch::new()
        .add(
            abi::collections_v2::functions::Creator {},
            collection_address,
        )
        .execute()
        .unwrap()
        .responses;

    let creator: String =
        match RpcBatch::decode::<_, abi::collections_v2::functions::Creator>(&responses[0]) {
            Some(data) => Hex(data).to_string(),
            None => {
                panic!("Failed to decode fee growth global 1x128");
            }
        };

    return creator;
}

pub fn get_collection_item_count(collection_address: Vec<u8>) -> Option<BigInt> {
    substreams::log::info!("get_collection_item_count2 {:?}", collection_address);
    let func = abi::collections_v2::functions::ItemsCount {};
    if let Some(items_result) = func.call(collection_address) {
        return Some(items_result);
    };

    return None;
}

pub fn get_collection_item(
    collection_address: Vec<u8>,
    item_id: u64,
) -> Option<(
    std::string::String,
    substreams::scalar::BigInt,
    substreams::scalar::BigInt,
    substreams::scalar::BigInt,
    Vec<u8>,
    std::string::String,
    std::string::String,
)> {
    let items = abi::collections_v2::functions::Items {
        param0: BigInt::from(item_id),
    };
    if let Some(items_result) = items.call(collection_address) {
        return Some(items_result);
    };

    return None;
}
