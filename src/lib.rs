mod abi;
mod constants;
mod data;
mod db;
mod macros;
mod pb;
mod rpc;
mod utils;

use data::constants::collections_v1;
use hex_literal::hex;
use pb::dcl;
use std::collections::HashMap;
use substreams::prelude::*;
use substreams::scalar::BigInt;
use substreams::{log, Hex};
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;

use crate::utils::sanitize_sql_string;

// Polygon's Contracts
const _MARKETPLACE_CONTRACT: [u8; 20] = hex!("02080031b45A3c67d338Dd4A2CC309D28756A160");
const MARKETPLACEV2_CONTRACT: [u8; 20] = hex!("480a0f4e360E8964e68858Dd231c2922f1df45Ef");
const COLLECTIONS_FACTORY: [u8; 20] = hex!("B549B2442b2BD0a53795BC5cDcBFE0cAF7ACA9f8");
const COLLECTIONS_V3_FACTORY: [u8; 20] = hex!("3195e88aE10704b359764CB38e429D24f1c2f781");

substreams_ethereum::init!();

/////// ---- COLLECTIONS  V1 ----- ///////

pub fn map_collections_v1(blk: eth::Block, contract_addresses: &[&[u8]]) -> pb::dcl::Collections {
    dcl::Collections {
        collections: blk
            .events::<abi::collection_factory::events::OwnershipTransferred>(contract_addresses)
            .map(|(event, log)| {
                let collection_address = Hex(log.address()).to_string();
                substreams::log::info!(
                    "OwnerTransferred event {:?} {:?}",
                    event,
                    collection_address
                );
                substreams::log::info!("at block {:?}", blk.number);

                let collection_data = rpc::collection_data_call(log.address().to_vec());
                dcl::Collection {
                    // address: Hex(event.address).to_string(),
                    address: collection_address.clone(),
                    owner: collection_data.4,
                    creator: collection_data.0,
                    name: collection_data.2,
                    symbol: collection_data.3,
                    is_completed: collection_data.5,
                    is_approved: collection_data.1,
                    is_editable: collection_data.6,
                    minters: ["123".to_string()].to_vec(),
                    managers: ["123".to_string()].to_vec(),
                    urn: utils::urn::get_urn_for_collection_v2(
                        collection_address,
                        "goerli".to_string(),
                    ),
                    created_at: blk.timestamp_seconds(),
                    reviewed_at: blk.timestamp_seconds(),
                    first_listed_at: None,
                    search_is_store_minter: false,
                }
            })
            .collect(),
    }
}

// Store addresses of the collections created by map_collection_created
#[substreams::handlers::store]
pub fn store_collections_v1_count(collections: dcl::Collections, store: StoreAddInt64) {
    for _collection in collections.collections {
        store.add(0, "count", 1); // we don't really care about the value, we'll just check if the key is present in the store
    }
}

// Workaround for Collections in Ethereum Goerli
#[substreams::handlers::map]
pub fn map_collection_created_ethereum_goerli(
    blk: eth::Block,
    // collections_v1_store: substreams::store::StoreGetString,
) -> Result<dcl::Collections, substreams::errors::Error> {
    Ok(map_collections_v1(
        blk,
        collections_v1::FIRST_COLLECTION_CREATED_GOERLI,
        // &[collections_v1::COLLECTIONS_GOERLI[0].as_bytes()],
    ))
    // let global = GLOBAL_VARIABLE.lock().unwrap();
    // substreams::log::info!("global {:?}", global);
    // if *global == 0 {
    //     Ok(map_collections_v1(
    //         blk,
    //         collections_v1::COLLECTIONS_GOERLI,
    //         // &[collections_v1::COLLECTIONS_GOERLI[0].as_bytes()],
    //     ))
    // } else {
    //     Ok(dcl::Collections {
    //         collections: vec![],
    //     })
    // }

    // if let None = collections_v1_store.get_last(collections_v1::COLLECTIONS_GOERLI[0]) {
    // Ok(map_collections_v1(
    //     blk,
    //     &[collections_v1::COLLECTIONS_GOERLI[0].as_bytes()],
    // ))
    // } else {
    //     Ok(dcl::Collections {
    //         collections: vec![],
    //     })
    // }
}

// // Workaround for Collections in Ethereum Mainnet
// #[substreams::handlers::map]
// pub fn map_collection_created_ethereum_mainnet(
//     blk: eth::Block,
//     collections_v1_store: substreams::store::StoreGetString,
// ) -> Result<dcl::Collections, substreams::errors::Error> {
//     if let None = collections_v1_store.get_last(collections_v1::COLLECTIONS_MAINNET[0]) {
//         Ok(map_collections_v1(
//             blk,
//             &[collections_v1::COLLECTIONS_MAINNET[0].as_bytes()],
//         ))
//     } else {
//         Ok(dcl::Collections {
//             collections: vec![],
//         })
//     }
// }

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
            ])
            .map(|(event, _log)| {
                substreams::log::info!("Collection created {:?}", event);
                let collection_data = rpc::collection_data_call(event.address.clone()); //@TODO avoid clone?
                dcl::Collection {
                    address: Hex(event.address.clone()).to_string(),
                    // creator: collection_data.0,
                    // is_approved: collection_data.1,
                    // name: collection_data.2,
                    // symbol: collection_data.3,
                    // owner: collection_data.4,
                    // is_completed: collection_data.5,
                    // created_at: blk.timestamp_seconds(),
                    owner: collection_data.4,
                    creator: collection_data.0,
                    name: collection_data.2,
                    symbol: collection_data.3,
                    is_completed: collection_data.5,
                    is_approved: collection_data.1,
                    is_editable: collection_data.6,
                    minters: [].to_vec(),
                    managers: [].to_vec(),
                    urn: utils::urn::get_urn_for_collection_v2(
                        Hex(event.address).to_string(),
                        "polygon".to_string(),
                    ),
                    created_at: blk.timestamp_seconds(),
                    reviewed_at: blk.timestamp_seconds(),
                    first_listed_at: None,
                    search_is_store_minter: false,
                }
            })
            .collect(),
    })
}

/// Store addresses of the collections created by map_collection_created
#[substreams::handlers::store]
pub fn store_collections(collections: dcl::Collections, store: StoreSetString) {
    for collection in collections.collections {
        store.set(0, collection.address, &"".to_string()); // we don't really care about the value, we'll just check if the key is present in the store
    }
}

/// Reads Issue events from the contract
#[substreams::handlers::map]
pub fn map_issues(
    blk: eth::Block,
    collections_store: substreams::store::StoreGetString,
) -> Result<dcl::NfTs, substreams::errors::Error> {
    let mut nfts = vec![];
    for trx in blk.transactions() {
        for call in trx.calls.iter() {
            let _call_index = call.index;
            if call.state_reverted {
                continue;
            }

            for log in call.logs.iter() {
                let collection_address = &Hex(log.clone().address).to_string();
                if let Some(_collection) = collections_store.get_last(collection_address) {
                    if let Some(event) = abi::collections_v2::events::Issue::match_and_decode(log) {
                        let timestamp = blk.timestamp_seconds().to_string();
                        let nft = dcl::Nft {
                            beneficiary: Hex(&event.beneficiary).to_string(),
                            issued_id: Some(event.issued_id.into()),
                            item_id: utils::get_item_id(
                                Hex(log.address.clone()).to_string(),
                                event.item_id.to_string(),
                            ),
                            token_id: Some(event.token_id.into()),
                            collection_address: Hex(log.address.clone()).to_string(),
                            created_at: timestamp.clone(),
                            updated_at: timestamp,
                        };
                        nfts.push(nft);
                    }
                }
            }
        }
    }
    Ok(dcl::NfTs { nfts })
}

/////// ---- ITEMS V1 ----- ///////

#[substreams::handlers::map]
pub fn map_add_items_v1(
    blk: eth::Block,
    collections: dcl::Collections,
) -> Result<dcl::Items, substreams::errors::Error> {
    Ok(dcl::Items {
        items: blk
            .events::<abi::ERC721::events::AddWearable>(collections_v1::COLLECTIONS_GOERLI_HEX)
            .map(|(add_item_event, log)| {
                let item = add_item_event.wearable_id;
                let representation = data::collections::find_wearable(&item).unwrap();
                // sanitize_sql_string(metadata.clone());
                dcl::Item {
                    id: utils::get_item_id(
                        Hex(log.address()).to_string(),
                        add_item_event.item_id.to_string(),
                    ),
                    rarity: representation.rarity,
                    max_supply: Some(add_item_event.max_issuance.into()),
                    total_supply: Some(dcl::BigInt {
                        value: "0".to_string(),
                    }),
                    price: Some(dcl::BigInt {
                        value: "0".to_string(),
                    }),
                    beneficiary: Hex(constants::ZERO_ADDRESS).to_string(),
                    metadata: metadata.clone(),
                    // content_hash: None,
                    blockchain_item_id: Some(add_item_event.item_id.into()),
                    collection_id: Hex(log.address()).to_string(),
                    created_at: blk.timestamp_seconds(),
                    item_type: utils::items::build_item_metadata(metadata).item_type,
                }
            })
            .collect(),
    })
}

/////// ---- ITEMS V2 ----- ///////

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
                    metadata: metadata.clone(),
                    content_hash,
                    blockchain_item_id: Some(add_item_event.item_id.into()),
                    collection_id: Hex(log.address()).to_string(),
                    created_at: blk.timestamp_seconds(),
                    item_type: utils::items::build_item_metadata(metadata).item_type,
                }
            })
            .collect(),
    })
}

// Reads the `Complete` event from the Collection contract and the items from it
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
            .flat_map(|(_complete_event, log)| {
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
                            metadata: item.5.clone(),
                            content_hash: item.6,
                            blockchain_item_id: Some(dcl::BigInt {
                                value: n.to_string(),
                            }),
                            collection_id: Hex(log.address()).to_string(),
                            created_at: blk.timestamp_seconds(),
                            item_type: utils::items::build_item_metadata(item.5).item_type,
                        }),
                        None => continue,
                    }
                }
                items
            })
            .collect::<Vec<dcl::Item>>(),
    })
}

// ORDERS
// MarketplaceV2

// Reads the Marketplacev2 order creation by the `OrderCreated` event
#[substreams::handlers::map]
pub fn map_order_created(blk: eth::Block) -> Result<dcl::Orders, substreams::errors::Error> {
    let mut order_map: HashMap<String, dcl::Order> = HashMap::new();

    blk.events::<abi::marketplacev2::events::OrderCreated>(&[&MARKETPLACEV2_CONTRACT])
        .for_each(|(event, log)| {
            substreams::log::info!("Order created {:?}", event);

            let id = Hex(event.id).to_string();

            // Check if the order already exists in the map
            if let Some(_existing_order) = order_map.get_mut(&id) {
                return; // Skip creating a new order with the same Id
            }

            let order = dcl::Order {
                id: id.clone(),
                marketplace_address: Hex(log.address()).to_string(),
                status: String::from(utils::orders::ORDER_OPEN),
                // TODO: add nft_address
                // TODO: add item
                nft_id: format!("{}-{}", Hex(event.nft_address).to_string(), event.asset_id),
                token_id: Some(dcl::BigInt {
                    value: event.asset_id.to_string(),
                }),
                price: Some(dcl::BigInt {
                    value: event.price_in_wei.to_string(),
                }),
                buyer: Hex("").to_string(),
                expires_at: Some(dcl::BigInt {
                    value: event.expires_at.to_string(),
                }),
                owner: Hex(event.seller).to_string(),
                tx_hash: Hex(log.receipt.transaction.hash.clone()).to_string(),
                block_number: blk.number,
                updated_at: blk.timestamp_seconds(),
                // TODO: add created_at
            };

            order_map.insert(id, order);
        });

    let orders = order_map.values().cloned().collect();

    Ok(dcl::Orders { orders })
}

/// Store addresses of the orders by nft_id created by map_order_created
#[substreams::handlers::store]
pub fn store_orders(orders: dcl::Orders, store: StoreSetString) {
    for order in orders.orders {
        store.set(0, order.nft_id, &order.id);
    }
}

// Reads the Marketplacev2 order execution by the `OrderSuccessful` event
#[substreams::handlers::map]
pub fn map_order_executed(blk: eth::Block) -> Result<dcl::Orders, substreams::errors::Error> {
    Ok(dcl::Orders {
        orders: blk
            .events::<abi::marketplacev2::events::OrderSuccessful>(&[&MARKETPLACEV2_CONTRACT])
            .map(|(event, log)| {
                substreams::log::info!("Order executed {:?}", event);
                dcl::Order {
                    id: Hex(event.id).to_string(),
                    marketplace_address: Hex(log.address()).to_string(),
                    status: String::from(utils::orders::ORDER_SOLD),
                    nft_id: Hex(event.nft_address).to_string(),
                    token_id: None,
                    price: Some(dcl::BigInt {
                        value: event.total_price.to_string(),
                    }),
                    buyer: Hex(event.buyer).to_string(),
                    expires_at: None,
                    owner: Hex(event.seller).to_string(),
                    tx_hash: Hex(log.receipt.transaction.hash.clone()).to_string(),
                    block_number: blk.number,
                    updated_at: blk.timestamp_seconds(),
                }
            })
            .collect(),
    })
}

// Reads the Marketplacev2 order execution by the `OrderCancelled` event
#[substreams::handlers::map]
pub fn map_order_cancelled(blk: eth::Block) -> Result<dcl::Orders, substreams::errors::Error> {
    Ok(dcl::Orders {
        orders: blk
            .events::<abi::marketplacev2::events::OrderCancelled>(&[&MARKETPLACEV2_CONTRACT])
            .map(|(event, log)| {
                substreams::log::info!("Order cancelled {:?}", event);
                dcl::Order {
                    id: Hex(event.id).to_string(),
                    marketplace_address: Hex(log.address()).to_string(),
                    status: String::from(utils::orders::ORDER_CANCELLED),
                    nft_id: Hex(event.nft_address).to_string(),
                    token_id: None,
                    price: None,
                    expires_at: None,
                    buyer: Hex("").to_string(),
                    owner: Hex(event.seller).to_string(),
                    tx_hash: Hex(log.receipt.transaction.hash.clone()).to_string(),
                    block_number: blk.number,
                    updated_at: blk.timestamp_seconds(),
                }
            })
            .collect(),
    })
}

#[substreams::handlers::map]
fn db_out_polygon(
    collections: dcl::Collections,
    items: dcl::Items,
    nfts: dcl::NfTs,
    orders: dcl::Orders,
    orders_executed: dcl::Orders,
    orders_cancelled: dcl::Orders,
    orders_store: substreams::store::StoreGetString,
) -> Result<DatabaseChanges, substreams::errors::Error> {
    let mut database_changes: DatabaseChanges = Default::default();
    log::info!("In db out collections {:?}", collections);

    db::collections::transform_collection_database_changes(&mut database_changes, collections);
    log::info!("In db out items {:?}", items);
    db::items::transform_item_database_changes(&mut database_changes, items);
    log::info!("In db out nfts {:?}", nfts);
    db::nfts::transform_nfts_database_changes(&mut database_changes, nfts);
    log::info!("In db out orders {:?}", orders);
    db::orders::transform_orders_database_changes(&mut database_changes, orders.clone());
    //@TODO move this to a fn to keep db_out clean
    if !orders.orders.is_empty() {
        for order in orders.orders {
            match orders_store.get_first(order.nft_id) {
                Some(active_order) => {
                    log::info!("There's an active order in the store {:?}", active_order);
                    db::orders::cancel_nft_order(&mut database_changes, active_order)
                }
                None => continue,
            }
        }
    }
    log::info!("In db out orders_executed {:?}", orders_executed);
    db::orders::transform_orders_status_database_changes(&mut database_changes, orders_executed);
    log::info!("In db out orders_cancelled {:?}", orders_cancelled);
    db::orders::transform_orders_status_database_changes(&mut database_changes, orders_cancelled);
    Ok(database_changes)
}

fn initiate_v1_collections() -> Vec<dcl::Collection> {
    let mut collections: Vec<dcl::Collection> = Vec::new();
    let all_collections_addresses = collections_v1::COLLECTIONS_GOERLI;
    for collection in all_collections_addresses {
        let collection_data = rpc::collection_data_call(collection.as_bytes().to_vec());

        collections.push(dcl::Collection {
            // address: Hex(event.address).to_string(),
            address: collection.to_string(),
            owner: collection_data.4,
            creator: collection_data.0,
            name: collection_data.2,
            symbol: collection_data.3,
            is_completed: collection_data.5,
            is_approved: collection_data.1,
            is_editable: collection_data.6,
            minters: ["123".to_string()].to_vec(),
            managers: ["123".to_string()].to_vec(),
            urn: utils::urn::get_urn_for_collection_v2(
                collection.to_string(),
                "goerli".to_string(),
            ),
            created_at: blk.timestamp_seconds(),
            reviewed_at: blk.timestamp_seconds(),
            first_listed_at: None,
            search_is_store_minter: false,
        })
    }
    collections
}

#[substreams::handlers::map]
fn db_out(
    collections: dcl::Collections,
    items: dcl::Items,
    nfts: dcl::NfTs,
    orders: dcl::Orders,
    orders_executed: dcl::Orders,
    orders_cancelled: dcl::Orders,
    orders_store: substreams::store::StoreGetString,
    collections_v1_goerli_store: substreams::store::StoreGetInt64,
) -> Result<DatabaseChanges, substreams::errors::Error> {
    let mut database_changes: DatabaseChanges = Default::default();
    log::info!("In db out collections {:?}", collections);
    let count = collections_v1_goerli_store.get_at(0, "count".to_string());
    log::info!("In db out count {:?}", count);
    if count == Some(1) {
        // we got the first collection created, now add the rest programmatically
        let rest_of_collections = initiate_v1_collections();
        db::collections::transform_collection_database_changes(&mut database_changes, collections);
    }
    log::info!("In db out items {:?}", items);
    db::items::transform_item_database_changes(&mut database_changes, items);
    log::info!("In db out nfts {:?}", nfts);
    db::nfts::transform_nfts_database_changes(&mut database_changes, nfts);
    log::info!("In db out orders {:?}", orders);
    db::orders::transform_orders_database_changes(&mut database_changes, orders.clone());
    //@TODO move this to a fn to keep db_out clean
    if !orders.orders.is_empty() {
        for order in orders.orders {
            match orders_store.get_first(order.nft_id) {
                Some(active_order) => {
                    log::info!("There's an active order in the store {:?}", active_order);
                    db::orders::cancel_nft_order(&mut database_changes, active_order)
                }
                None => continue,
            }
        }
    }
    log::info!("In db out orders_executed {:?}", orders_executed);
    db::orders::transform_orders_status_database_changes(&mut database_changes, orders_executed);
    log::info!("In db out orders_cancelled {:?}", orders_cancelled);
    db::orders::transform_orders_status_database_changes(&mut database_changes, orders_cancelled);
    Ok(database_changes)
}

// #[substreams::handlers::map]
// fn db_out_ethereum_goerli(
//     collections: dcl::Collections,
// ) -> Result<DatabaseChanges, substreams::errors::Error> {
//     let mut database_changes: DatabaseChanges = Default::default();
//     log::info!("In db out collections {:?}", collections);
//     db::collections::transform_collection_database_changes(&mut database_changes, collections);
//     Ok(database_changes)
// }
