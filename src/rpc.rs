use crate::abi;
use substreams::{scalar::BigInt, Hex};
use substreams_ethereum::rpc::RpcBatch;

pub fn collection_data_call(collection_address: Vec<u8>) -> String {
    // using RpcBatch since it will fetch more data in a batch later on
    match RpcBatch::new()
        .add(
            abi::collections_v2::functions::Creator {},
            collection_address,
        )
        .execute()
    {
        Ok(responses) => {
            let creator: String = match RpcBatch::decode::<_, abi::collections_v2::functions::Creator>(
                &responses.responses[0],
            ) {
                Some(data) => Hex(data).to_string(),
                None => {
                    panic!("Failed to decode fee growth global 1x128");
                }
            };

            creator
        }
        Err(_err) => String::from(""),
    }
}

pub fn get_collection_item_count(collection_address: Vec<u8>) -> Option<BigInt> {
    let func = abi::collections_v2::functions::ItemsCount {};
    func.call(collection_address)
}

type CollectionItemTuple = (
    std::string::String,
    substreams::scalar::BigInt,
    substreams::scalar::BigInt,
    substreams::scalar::BigInt,
    Vec<u8>,
    std::string::String,
    std::string::String,
);

pub fn get_collection_item(
    collection_address: Vec<u8>,
    item_id: u64,
) -> Option<CollectionItemTuple> {
    let items = abi::collections_v2::functions::Items {
        param0: BigInt::from(item_id),
    };
    items.call(collection_address)
}
