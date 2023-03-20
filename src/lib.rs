mod abi;
mod db;
mod macros;
mod pb;
mod rpc;
mod utils;

use hex_literal::hex;
use pb::dcl;
use substreams::prelude::*;
use substreams::scalar::BigInt;
use substreams::{log, store::StoreAppend, Hex};
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_ethereum::pb::eth::v2 as eth;

use crate::utils::sanitize_sql_string;

static COLLECTION_STORE_KEY: &str = "addresses";
const COLLECTIONS_FACTORY: [u8; 20] = hex!("B549B2442b2BD0a53795BC5cDcBFE0cAF7ACA9f8");
const COLLECTIONS_V3_FACTORY: [u8; 20] = hex!("3195e88aE10704b359764CB38e429D24f1c2f781");

substreams_ethereum::init!();

// Reads the collection creation by the `ProxyCreated` event
#[substreams::handlers::map]
pub fn map_collection_created(
    blk: eth::Block,
) -> Result<dcl::Collections, substreams::errors::Error> {
    Ok(dcl::Collections {
        collections: blk
            .events::<abi::collection_factoryv3::events::ProxyCreated>(&[
                &COLLECTIONS_FACTORY,
                &COLLECTIONS_V3_FACTORY,
            ]) //@TODO try out if the COLLECTIONS_FACTORY has the same TOPIC_0 and if it matches the event
            .map(|(event, _log)| {
                substreams::log::info!("Collection created {:?}", event);
                let creator = rpc::collection_data_call(event.address.clone()); //@TODO avoid clone?
                log::info!("creator: {}", dcl_hex!(creator));
                let address = format!("{}", Hex(event.address));
                dcl::Collection {
                    address,
                    creator: dcl_hex!(creator),
                    created_at: blk.timestamp_seconds(),
                }
            })
            .collect(),
    })
}

/// Store addresses of the collections created by map_collection_created
#[substreams::handlers::store]
pub fn store_collections(collections: dcl::Collections, store: StoreAppend<String>) {
    for collection in collections.collections {
        store.append(0, COLLECTION_STORE_KEY, collection.address); // since the store is a of a KV type, we're gonna store under the "address" key an string of contract addresses separated by comma
    }
}

/// Reads Issue events from the contract
#[substreams::handlers::map]
pub fn map_issues(
    blk: eth::Block,
    collections_store: substreams::store::StoreGetString,
) -> Result<dcl::Issues, substreams::errors::Error> {
    match collections_store.get_first(COLLECTION_STORE_KEY) {
        None => Ok(dcl::Issues { issues: vec![] }),
        Some(from_store_unwrapped) => {
            let _: Vec<_> = from_store_unwrapped
                .split(';')
                .map(|v| v.to_string())
                .inspect(|x| substreams::log::info!("element {}", x))
                .collect();

            let mut addresses = vec![];
            for address in from_store_unwrapped.split(';') {
                match hex::decode(address) {
                    Ok(decoded) => addresses.push(decoded),
                    Err(_err) => log::debug!("Err decoding address {}", address),
                }
            }
            //@TODO: see why the pop is needed
            addresses.pop();
            let mut addresses_ref = vec![];
            for address in &addresses {
                addresses_ref.push(address.as_slice());
            }

            Ok(dcl::Issues {
                issues: blk
                    .events::<abi::collections_v2::events::Issue>(&addresses_ref)
                    .map(|(issue_event, _log)| {
                        log::info!("NFT Issue seen {:?}", issue_event);

                        dcl::Issue {
                            beneficiary: Hex(&issue_event.beneficiary).to_string(),
                            issued_id: Some(issue_event.issued_id.into()),
                            item_id: Some(issue_event.item_id.into()),
                            token_id: Some(issue_event.token_id.into()),
                        }
                    })
                    .collect(),
            })
        }
    }
}

/// Reads item creationg by the `AddItem` event
#[substreams::handlers::map]
pub fn map_add_items(
    blk: eth::Block,
    collections: dcl::Collections,
) -> Result<dcl::Items, substreams::errors::Error> {
    let mut addresses = vec![];
    for collection in collections.collections {
        match hex::decode(&collection.address) {
            Ok(decoded) => addresses.push(decoded),
            Err(_err) => log::debug!("Err decoding the collection address {}", collection.address),
        }
    }
    let mut addresses_ref = vec![];
    for address in &addresses {
        addresses_ref.push(address.as_slice());
    }

    Ok(dcl::Items {
        items: blk
            .events::<abi::collections_v2_fixed::events::AddItem>(&addresses_ref)
            .map(|(add_item_event, log)| {
                log::info!("INFO: Add item found {:?}", add_item_event);
                let item = add_item_event.item;
                //@TODO missing fields:
                // creation_fee => grab from oracle
                // creator? => it can be part of the collection
                let abi::collections_v2_fixed::events::Erc721BaseCollectionV2Item {
                    beneficiary,
                    rarity,
                    max_supply,
                    total_supply,
                    price,
                    metadata,
                    content_hash,
                } = item;
                sanitize_sql_string(metadata.clone());
                dcl::Item {
                    id: utils::get_item_id(
                        Hex(log.address()).to_string(),
                        add_item_event.item_id.to_string(),
                    ),
                    rarity,
                    max_supply: Some(max_supply.into()),
                    total_supply: Some(total_supply.into()),
                    price: Some(price.into()),
                    beneficiary: Hex(beneficiary).to_string(),
                    metadata,
                    content_hash,
                    blockchain_item_id: Some(add_item_event.item_id.into()),
                    collection_id: Hex(log.address()).to_string(),
                    created_at: blk.timestamp_seconds(),
                }
            })
            .collect(),
    })
}

// Reads the `Complete` event from the Collection contrat and the items from it
#[substreams::handlers::map]
pub fn map_collection_complete(
    blk: eth::Block,
    collections: dcl::Collections,
) -> Result<dcl::Items, substreams::errors::Error> {
    let mut addresses = vec![];
    for collection in collections.collections {
        match hex::decode(&collection.address) {
            Ok(decoded) => addresses.push(decoded),
            Err(_err) => log::debug!("Err decoding address {}", collection.address),
        }
    }
    let mut addresses_ref = vec![];
    for address in &addresses {
        addresses_ref.push(address.as_slice());
    }
    Ok(dcl::Items {
        items: blk
            .events::<abi::collections_v2::events::Complete>(&addresses_ref)
            .flat_map(|(complete_event, log)| {
                log::info!("Complete event found! {:?}", complete_event);
                let collection_item_count = rpc::get_collection_item_count(log.address().to_vec());
                let mut items = vec![];
                let item_amount =
                    BigInt::to_u64(&collection_item_count.unwrap_or_else(BigInt::zero));
                for n in 0..item_amount {
                    match rpc::get_collection_item(log.address().to_vec(), n) {
                        Some(item) => items.push(dcl::Item {
                            id: utils::get_item_id(Hex(log.address()).to_string(), n.to_string()),
                            rarity: item.0,
                            max_supply: Some(item.1.into()),
                            total_supply: Some(dcl::BigInt {
                                value: item.2.to_string(),
                            }),
                            price: Some(dcl::BigInt {
                                value: item.3.to_string(),
                            }),
                            beneficiary: Hex(item.4).to_string(),
                            metadata: item.5,
                            content_hash: item.6,
                            blockchain_item_id: Some(dcl::BigInt {
                                value: n.to_string(),
                            }),
                            collection_id: Hex(log.address()).to_string(),
                            created_at: blk.timestamp_seconds(),
                        }),
                        None => continue,
                    }
                }
                items
            })
            .collect::<Vec<dcl::Item>>(),
    })
}

#[substreams::handlers::map]
fn db_out(
    collections: dcl::Collections,
    items: dcl::Items,
) -> Result<DatabaseChanges, substreams::errors::Error> {
    let mut database_changes: DatabaseChanges = Default::default();
    log::info!("In db out collections {:?}", collections);
    db::transform_collection_database_changes(&mut database_changes, collections);
    log::info!("In db out items {:?}", items);
    db::transform_item_database_changes(&mut database_changes, items);
    Ok(database_changes)
}
