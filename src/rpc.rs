use crate::abi::{self};
use substreams::{scalar::BigInt, Hex};
use substreams_ethereum::rpc::RpcBatch;

// (Creator, is_approved, Name, Symbol, Owner, is_complete, is_editable, itemsCount )
pub type CollectionDataTuple = (String, bool, String, String, String, bool, bool, String);

pub fn collection_data_call(collection_address: Vec<u8>) -> CollectionDataTuple {
    // using RpcBatch since it will fetch more data in a batch later on
    match RpcBatch::new()
        .add(
            abi::collections_v2::functions::Creator {},
            collection_address.clone(),
        )
        .add(
            abi::collections_v2::functions::IsApproved {},
            collection_address.clone(),
        )
        .add(
            abi::collections_v2::functions::Name {},
            collection_address.clone(),
        )
        .add(
            abi::collections_v2::functions::Symbol {},
            collection_address.clone(),
        )
        .add(
            abi::collections_v2::functions::Owner {},
            collection_address.clone(),
        )
        .add(
            abi::collections_v2::functions::IsCompleted {},
            collection_address.clone(),
        )
        .add(
            abi::collections_v2::functions::IsEditable {},
            collection_address.clone(),
        )
        .add(
            abi::collections_v2::functions::BaseUri {},
            collection_address,
        )
        .execute()
    {
        Ok(responses) => {
            let creator = RpcBatch::decode::<_, abi::collections_v2::functions::Creator>(
                &responses.responses[0],
            )
            .unwrap_or_else(|| String::from("").as_bytes().to_vec());
            let is_approved = RpcBatch::decode::<_, abi::collections_v2::functions::IsApproved>(
                &responses.responses[1],
            )
            .unwrap_or(false);
            let name = RpcBatch::decode::<_, abi::collections_v2::functions::Name>(
                &responses.responses[2],
            )
            .unwrap_or_else(|| String::from(""));
            let symbol = RpcBatch::decode::<_, abi::collections_v2::functions::Symbol>(
                &responses.responses[3],
            )
            .unwrap_or_else(|| String::from(""));
            let owner = RpcBatch::decode::<_, abi::collections_v2::functions::Owner>(
                &responses.responses[4],
            )
            .unwrap_or_else(|| String::from("").as_bytes().to_vec());
            let is_completed = RpcBatch::decode::<_, abi::collections_v2::functions::IsCompleted>(
                &responses.responses[5],
            )
            .unwrap_or(false);
            let is_editable = RpcBatch::decode::<_, abi::collections_v2::functions::IsEditable>(
                &responses.responses[6],
            )
            .unwrap_or(false);
            let base_uri = RpcBatch::decode::<_, abi::collections_v2::functions::BaseUri>(
                &responses.responses[7],
            )
            .unwrap_or(String::from(""));

            (
                Hex(creator).to_string(),
                is_approved,
                name,
                symbol,
                Hex(owner).to_string(),
                is_completed,
                is_editable,
                base_uri,
            )
        }
        Err(_err) => (
            String::from(""),
            false,
            String::from(""),
            String::from(""),
            String::from(""),
            false,
            false,
            String::from(""),
        ),
    }
}

pub fn get_token_uri(collection_address: Vec<u8>, token_id: BigInt) -> Option<String> {
    let func = abi::erc721::functions::TokenUri { token_id };
    func.call(collection_address)
}

// pub fn get_collection_v1_item_count(collection_address: Vec<u8>) -> Option<BigInt> {
//     let func = abi::erc721::functions::ItemsCount {};
//     func.call(collection_address)
// }

// type CollectionItemTuple = (
//     std::string::String,
//     substreams::scalar::BigInt,
//     substreams::scalar::BigInt,
//     substreams::scalar::BigInt,
//     Vec<u8>,
//     std::string::String,
//     std::string::String,
// );

// pub fn get_collection_item(
//     collection_address: Vec<u8>,
//     item_id: u64,
// ) -> Option<CollectionItemTuple> {
//     let items = abi::collections_v2::functions::Items {
//         param0: BigInt::from(item_id),
//     };
//     items.call(collection_address)
// }
