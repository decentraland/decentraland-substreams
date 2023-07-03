mod abi;
mod constants;
mod data;
mod db;
mod macros;
mod pb;
mod rpc;
mod utils;

use constants::{
    COLLECTIONS_FACTORY, COLLECTIONS_V3_FACTORY, MARKETPLACEV2_CONTRACT,
    MARKETPLACE_GOERLI_CONTRACT, MARKETPLACE_MAINNET_CONTRACT,
};
use data::constants::collections_v1;
use pb::dcl;
use std::collections::HashMap;
use substreams::prelude::*;
use substreams::scalar::BigInt;
use substreams::{log, Hex};
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;

use crate::utils::sanitize_sql_string;

substreams_ethereum::init!();

/////// ---- ITEMS V1 ----- ///////
#[substreams::handlers::map]
pub fn map_add_items_v1(
    network: String,
    blk: eth::Block,
) -> Result<dcl::Items, substreams::errors::Error> {
    let contracts = if network == "goerli" {
        collections_v1::COLLECTIONS_GOERLI_HEX
    } else {
        collections_v1::COLLECTIONS_MAINNET
    };
    Ok(dcl::Items {
        items: blk
            .events::<abi::erc721::events::AddWearable>(contracts)
            .map(|(add_item_event, log)| {
                substreams::log::info!("Add AddWearable! {:?}", add_item_event);
                let representation =
                    data::collections::find_wearable(&add_item_event.wearable_id).unwrap();
                let collection_address = Hex(log.address()).to_string();
                let collection_data = rpc::collection_data_call(log.address().to_vec());
                let item_urn = utils::urn::get_urn_for_wearable_v1(
                    &network,
                    &collection_data.2,
                    &representation.id,
                );
                dcl::Item {
                    creator: collection_data.4, // the creator for v1 is the collection owner
                    id: utils::get_item_id(
                        Hex(log.address()).to_string(),
                        add_item_event.wearable_id.to_string(),
                    ),
                    blockchain_id: 0, // This will be set correctly by the db_out module that will read the counter from the store
                    collection: collection_address.clone(),
                    creation_fee: Some(dcl::BigInt {
                        value: "0".to_string(),
                    }),
                    rarity: representation.rarity.clone(),
                    available: Some(add_item_event.max_issuance.clone().into()),
                    total_supply: Some(dcl::BigInt {
                        value: "0".to_string(),
                    }),
                    max_supply: Some(add_item_event.max_issuance.into()),
                    price: Some(dcl::BigInt {
                        value: "0".to_string(),
                    }),
                    beneficiary: constants::ZERO_ADDRESS.to_string(),
                    raw_metadata: String::new(),         // not used in v1
                    search_is_collection_approved: true, // not used for v1
                    minters: [].to_vec(),                // not used for v1
                    managers: [].to_vec(),               // not used for v1
                    uri: collection_data.7,
                    urn: item_urn.clone(),
                    image: utils::items::get_item_image(&item_urn),
                    created_at: blk.timestamp_seconds(),
                    updated_at: blk.timestamp_seconds(),
                    reviewed_at: blk.timestamp_seconds(),
                    search_is_store_minter: false, // not used for v1
                    metadata: Some(dcl::Metadata {
                        item_type: utils::items::WEARABLE_V1.to_string(),
                        id: representation.id.clone(),
                        wearable: Some(dcl::Wearable {
                            id: representation.id,
                            name: representation.name,
                            description: representation.description,
                            collection: collection_address,
                            category: representation.category,
                            rarity: representation.rarity,
                            body_shapes: representation.body_shapes,
                        }),
                        emote: None,
                    }),
                    item_type: utils::items::WEARABLE_V1.to_string(),
                    content_hash: None,    // not used for v1
                    first_listed_at: None, // not used for v1
                    sold_at: None,         // not used for v1
                }
            })
            .collect(),
    })
}

/// Store items available amount
#[substreams::handlers::store]
pub fn store_collections_items_available(items: dcl::Items, store: StoreSetString) {
    for item in items.items {
        store.set(0, item.id, &item.available.unwrap().value);
    }
}

#[substreams::handlers::map]
pub fn map_add_collections_v1(
    items: dcl::Items,
    collections_v1_store: StoreGetInt64,
) -> Result<dcl::Collections, substreams::errors::Error> {
    substreams::log::info!("map_add_collections_v1 items:! {:?}", items);
    if items.items.is_empty() {
        return Ok(dcl::Collections {
            collections: vec![],
        });
    }

    let item = &items.items[0];
    let collection_address = item.collection.clone();
    let store_value = collections_v1_store.get_last(collection_address.clone());
    substreams::log::info!("map_add_collections_v1: store_value {:?}", store_value);
    if store_value == Some(1) {
        substreams::log::info!("map_add_collections_v1 inside get_last is none");
        let collection_data =
            rpc::collection_data_call(Hex::decode(collection_address.clone()).unwrap());
        Ok(dcl::Collections {
            collections: [dcl::Collection {
                address: collection_address.clone(),
                owner: collection_data.4,
                creator: collection_data.0,
                name: collection_data.2,
                symbol: collection_data.3,
                is_completed: collection_data.5,
                is_approved: collection_data.1,
                is_editable: collection_data.6,
                minters: [].to_vec(),  //@TODO update this logic
                managers: [].to_vec(), //@TODO update this logic
                urn: utils::urn::get_urn_for_collection_v2(
                    &collection_address,
                    "goerli".to_string(), //@TODO: unharcode this
                ),
                created_at: item.created_at,
                updated_at: item.updated_at,
                reviewed_at: item.reviewed_at,
                first_listed_at: None,
                search_is_store_minter: false,
            }]
            .to_vec(),
        })
    } else {
        Ok(dcl::Collections {
            collections: vec![],
        })
    }
}

/// Store a counter by collection address to know if the collection has been already created
#[substreams::handlers::store]
pub fn store_collections_v1(items: dcl::Items, store: StoreAddInt64) {
    for item in items.items {
        substreams::log::info!(
            "store_collections_v1 saving item.collection {:?}",
            item.collection
        );
        store.add(0, item.collection, 1);
    }
}

/// Reads Transfer events from the Collection v1 contracts
#[substreams::handlers::map]
pub fn map_issues_v1(
    blk: eth::Block,
    store_collections_v1: substreams::store::StoreGetString,
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
                if let Some(_collection) = store_collections_v1.get_last(collection_address) {
                    if let Some(event) = abi::erc721::events::Transfer3::match_and_decode(log) {
                        // @TODO: add this logic
                        // if (event.params.tokenId.toString() == '') {
                        //     return
                        //   }

                        substreams::log::info!("Transfer Event found!! {:?}", event);
                        let is_mint =
                            dcl_hex!(Hex(&event.from).to_string()) == *constants::ZERO_ADDRESS;
                        substreams::log::info!("is_mint: {:?}", is_mint);
                        if is_mint {
                            let token_uri =
                                rpc::get_token_uri(log.clone().address, event.token_id.clone());
                            let token_uri_unwrapped = &token_uri.clone().unwrap_or_default();
                            let issued_id =
                                utils::items::get_issued_id_from_token_uri(token_uri_unwrapped);
                            let wearable_id =
                                utils::items::get_wearable_id_from_token_uri(token_uri_unwrapped);
                            let representation =
                                data::collections::find_wearable(wearable_id).unwrap();
                            let item_id = utils::get_item_id(
                                Hex(log.address.clone()).to_string(),
                                representation.id,
                            );
                            substreams::log::info!("token_uri: {:?}", token_uri);
                            substreams::log::info!("issued_id: {:?}", issued_id);
                            substreams::log::info!("wearable_id: {:?}", wearable_id);
                            substreams::log::info!("item_id: {:?}", item_id);
                            let timestamp = blk.timestamp_seconds().to_string();
                            let nft = dcl::Nft {
                                owner: Hex(&event.to).to_string(),
                                beneficiary: Hex(&event.to).to_string(),
                                issued_id: Some(dcl::BigInt {
                                    value: issued_id.to_string(),
                                }),
                                item_id,
                                token_id: Some(event.token_id.into()),
                                collection_address: Hex(log.address.clone()).to_string(),
                                created_at: timestamp.clone(),
                                updated_at: timestamp,
                            };
                            nfts.push(nft);
                        } else {
                            //@TODO: cancel active order
                        }
                    }
                }
            }
        }
    }
    Ok(dcl::NfTs { nfts })
}

/// Store the amount of nfts minted for the item id
#[substreams::handlers::store]
pub fn store_items_mints(nfts: dcl::NfTs, store: StoreAddInt64) {
    for nft in nfts.nfts {
        substreams::log::info!("store_items_mints saving item {:?}", nft.item_id);
        store.add(0, nft.item_id, 1);
    }
}

/////// ---- ITEMS V2 ----- ///////

/// Reads the collection creation by the `ProxyCreated` event
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
                        &Hex(event.address).to_string(),
                        "polygon".to_string(), //@TODO: fix me
                    ),
                    created_at: blk.timestamp_seconds(),
                    updated_at: blk.timestamp_seconds(),
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

// Reads the SetApproved Event for collections v2
#[substreams::handlers::map]
pub fn map_collection_set_approved_event(
    blk: eth::Block,
    collections_store: substreams::store::StoreGetString,
) -> Result<dcl::CollectionSetApprovedEvents, substreams::errors::Error> {
    let mut events = vec![];
    for trx in blk.transactions() {
        for call in trx.calls.iter() {
            let _call_index = call.index;
            if call.state_reverted {
                continue;
            }

            for log in call.logs.iter() {
                let collection_address = &Hex(log.clone().address).to_string();
                if let Some(_collection) = collections_store.get_last(collection_address) {
                    if let Some(event) =
                        abi::collections_v2::events::SetApproved::match_and_decode(log)
                    {
                        substreams::log::info!("SetApproved Event found! {:?}", event);
                        let timestamp = blk.timestamp_seconds().to_string();
                        let nft = dcl::CollectionSetApprovedEvent {
                            collection: collection_address.to_string(),
                            new_value: event.new_value,
                            updated_at: timestamp,
                        };
                        events.push(nft);
                    }
                }
            }
        }
    }
    Ok(dcl::CollectionSetApprovedEvents { events })
}

// Reads the SetGlobalMinter Event for collections v2
#[substreams::handlers::map]
pub fn map_collection_set_global_minter_event(
    blk: eth::Block,
    collections_store: substreams::store::StoreGetString,
) -> Result<dcl::CollectionSetGlobalMinterEvents, substreams::errors::Error> {
    let mut events = vec![];
    for trx in blk.transactions() {
        for call in trx.calls.iter() {
            let _call_index = call.index;
            if call.state_reverted {
                continue;
            }

            for log in call.logs.iter() {
                let collection_address = &Hex(log.clone().address).to_string();
                if let Some(_collection) = collections_store.get_last(collection_address) {
                    if let Some(event) =
                        abi::collections_v2::events::SetGlobalMinter::match_and_decode(log)
                    {
                        substreams::log::info!("SetGlobalMinter Event found! {:?}", event);
                        let timestamp = blk.timestamp_seconds().to_string();
                        let nft = dcl::CollectionSetGlobalMinterEvent {
                            collection: collection_address.to_string(),
                            minter: Hex(event.minter).to_string(),
                            timestamp,
                            value: event.value,
                        };
                        events.push(nft);
                    }
                }
            }
        }
    }
    Ok(dcl::CollectionSetGlobalMinterEvents { events })
}

// Reads the SetItemMinter Event for collections v2
#[substreams::handlers::map]
pub fn map_collection_set_item_minter_event(
    blk: eth::Block,
    collections_store: substreams::store::StoreGetString,
) -> Result<dcl::SetItemMinterEvents, substreams::errors::Error> {
    let mut events = vec![];
    for trx in blk.transactions() {
        for call in trx.calls.iter() {
            let _call_index = call.index;
            if call.state_reverted {
                continue;
            }

            for log in call.logs.iter() {
                let collection_address = &Hex(log.clone().address).to_string();
                // if let Some(_collection) = collections_store.get_last(collection_address) {
                if let Some(event) =
                    abi::collections_v2::events::SetItemMinter::match_and_decode(log)
                {
                    substreams::log::info!("SetItemMinter Event found! {:?}", event);
                    let timestamp = blk.timestamp_seconds().to_string();
                    let nft = dcl::SetItemMinterEvent {
                        item: event.item_id.to_string(),
                        collection: collection_address.to_string(),
                        minter: Hex(event.minter).to_string(),
                        timestamp,
                        value: event.value.to_string(),
                    };
                    events.push(nft);
                }
                // }
            }
        }
    }
    Ok(dcl::SetItemMinterEvents { events })
}

// /// Store addresses of the collections created by map_collection_created
// #[substreams::handlers::store]
// pub fn store_items_minters(
//     set_minter_events: dcl::SetItemMinterEvents,
//     store: StoreSetProto<dcl::ItemMinters>,
// ) {
//     for set_minter_event in set_minter_events.events {
//         store.set(
//             0,
//             set_minter_event.item,
//             dcl::ItemMinters {
//                 item: set_minter_event.item,
//                 minters: vec![dcl::Minter {
//                     address: set_minter_event.minter,
//                     value: set_minter_event.value,
//                 }],
//             },
//         ); // we don't really care about the value, we'll just check if the key is present in the store
//     }
// }

/// NFTS Collections V2
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
                            owner: Hex(&event.beneficiary).to_string(),
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

/// Store addresses of the collections created by map_collection_created
#[substreams::handlers::store]
pub fn store_nfts_item(nfts: dcl::NfTs, store: StoreSetString) {
    for nft in nfts.nfts {
        let nft_id = format!(
            "{}-{}",
            nft.collection_address,
            nft.token_id.clone().unwrap().value
        );
        store.set(0, nft_id, &nft.item_id);
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
                let item = add_item_event.item;
                let collection_data = rpc::collection_data_call(log.address().to_vec());
                //@TODO missing fields:
                // creation_fee => grab from oracle
                let collection_address = Hex(log.address()).to_string();
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
                let item_urn = utils::urn::get_urn_for_collection_v2(
                    &collection_address,
                    "mumbai".to_string(),
                );
                let item_id = utils::get_item_id(
                    Hex(log.address()).to_string(),
                    add_item_event.item_id.to_string(),
                );
                let mut item = dcl::Item {
                    id: item_id,
                    creator: collection_data.0,
                    blockchain_id: BigInt::to_u64(&add_item_event.item_id) as i64, //@TODO: check if works in runtime
                    collection: collection_address,
                    creation_fee: Some(dcl::BigInt {
                        value: "0".to_string(),
                    }),
                    rarity,
                    available: Some(max_supply.clone().into()),
                    total_supply: Some(total_supply.into()),
                    max_supply: Some(max_supply.into()),
                    price: Some(price.into()),
                    beneficiary: Hex(beneficiary).to_string(),
                    content_hash: Some(content_hash),
                    raw_metadata: metadata.clone(),
                    search_is_collection_approved: collection_data.1,
                    minters: [].to_vec(),  //@TODO update this logic
                    managers: [].to_vec(), //@TODO update this logic
                    uri: collection_data.7,
                    urn: item_urn.clone(),
                    image: utils::items::get_item_image(&item_urn),
                    created_at: blk.timestamp_seconds(),
                    updated_at: blk.timestamp_seconds(),
                    reviewed_at: blk.timestamp_seconds(),
                    search_is_store_minter: false,
                    metadata: None, // it gets set later
                    item_type: utils::items::get_item_type_from_metadata(metadata).item_type,
                    sold_at: None,
                    first_listed_at: None, //@TODO: Add this logic
                };
                item.metadata = Some(utils::items::build_metadata(&item));
                item
            })
            .collect(),
    })
}

// ORDERS

fn get_marketplace_contract(network: String) -> [u8; 20] {
    if network == "goerli" {
        MARKETPLACE_GOERLI_CONTRACT
    } else if network == "mainnet" {
        MARKETPLACE_MAINNET_CONTRACT
    } else {
        MARKETPLACEV2_CONTRACT
    }
}

// Reads the Marketplacev2 order creation by the `OrderCreated` event
#[substreams::handlers::map]
pub fn map_order_created(
    network: String,
    blk: eth::Block,
) -> Result<dcl::Orders, substreams::errors::Error> {
    let mut order_map: HashMap<String, dcl::Order> = HashMap::new();
    let contract = get_marketplace_contract(network);
    blk.events::<abi::marketplace::events::OrderCreated>(&[&contract])
        .for_each(|(event, log)| {
            substreams::log::info!("Order created {:?}", event);
            let id: String = Hex(event.id).to_string();
            // Check if the order already exists in the map
            if let Some(_existing_order) = order_map.get_mut(&id) {
                return; // Skip creating a new order with the same Id
            }

            let order = dcl::Order {
                id: id.clone(),
                marketplace_address: Hex(log.address()).to_string(),
                status: String::from(utils::orders::ORDER_OPEN),
                nft_address: Hex(event.nft_address.clone()).to_string(),
                nft: format!("{}-{}", Hex(event.nft_address).to_string(), event.asset_id),
                token_id: Some(dcl::BigInt {
                    value: event.asset_id.to_string(),
                }),
                item: String::from(""), // Item field is then set when inserting in the db
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
                created_at: blk.timestamp_seconds(),
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
        store.set(0, order.nft, &order.id);
    }
}

// Reads the Marketplacev2 order execution by the `OrderSuccessful` event
#[substreams::handlers::map]
pub fn map_order_executed(
    network: String,
    blk: eth::Block,
) -> Result<dcl::Orders, substreams::errors::Error> {
    Ok(dcl::Orders {
        orders: blk
            .events::<abi::marketplacev2::events::OrderSuccessful>(&[&get_marketplace_contract(
                network,
            )])
            .map(|(event, log)| {
                substreams::log::info!("Order executed {:?}", event);
                dcl::Order {
                    id: Hex(event.id).to_string(),
                    marketplace_address: Hex(log.address()).to_string(),
                    status: String::from(utils::orders::ORDER_SOLD),
                    nft_address: Hex(event.nft_address.clone()).to_string(),
                    nft: format!("{}-{}", Hex(event.nft_address).to_string(), event.asset_id),
                    item: String::from(""), //@TODO: fix me
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
                    created_at: blk.timestamp_seconds(),
                }
            })
            .collect(),
    })
}

// Reads the Marketplacev2 order execution by the `OrderCancelled` event
#[substreams::handlers::map]
pub fn map_order_cancelled(
    network: String,
    blk: eth::Block,
) -> Result<dcl::Orders, substreams::errors::Error> {
    Ok(dcl::Orders {
        orders: blk
            .events::<abi::marketplacev2::events::OrderCancelled>(&[&get_marketplace_contract(
                network,
            )])
            .map(|(event, log)| {
                substreams::log::info!("Order cancelled {:?}", event);
                dcl::Order {
                    id: Hex(event.id).to_string(),
                    marketplace_address: Hex(log.address()).to_string(),
                    status: String::from(utils::orders::ORDER_CANCELLED),
                    nft_address: Hex(event.nft_address.clone()).to_string(),
                    nft: format!("{}-{}", Hex(event.nft_address).to_string(), event.asset_id),
                    item: String::from(""), //@TODO: fix me
                    token_id: None,
                    price: None,
                    expires_at: None,
                    buyer: Hex("").to_string(),
                    owner: Hex(event.seller).to_string(),
                    tx_hash: Hex(log.receipt.transaction.hash.clone()).to_string(),
                    block_number: blk.number,
                    updated_at: blk.timestamp_seconds(),
                    created_at: blk.timestamp_seconds(),
                }
            })
            .collect(),
    })
}

// Outs

#[substreams::handlers::map]
fn db_out(
    network: String,
    collections: dcl::Collections,
    items: dcl::Items,
    nfts: dcl::NfTs,
    orders: dcl::Orders,
    orders_executed: dcl::Orders,
    orders_cancelled: dcl::Orders,
    store_orders: StoreGetString,
    store_nfts_item: StoreGetString,
    store_collections_items_available: StoreGetString,
    store_items_mints: StoreGetInt64,
    store_collections_v1: StoreGetInt64,
) -> Result<DatabaseChanges, substreams::errors::Error> {
    // let mut database_changes: DatabaseChanges = Default::default();
    let mut tables = substreams_database_change::tables::Tables::new();
    // Collections
    log::info!("In db out collections {:?}", collections);
    db::collections::transform_collection_database_changes(&mut tables, collections);
    // Items
    log::info!("In db out items {:?}", items);
    db::items::transform_item_database_changes(network, &mut tables, items, store_collections_v1);
    // NFTs
    log::info!("In db out nfts {:?}", nfts);
    db::nfts::transform_nfts_database_changes(&mut tables, nfts.clone());
    // get the item available count based on nfts minted
    db::items::update_item_available(
        &mut tables,
        nfts,
        store_items_mints,
        store_collections_items_available,
    );
    // Orders
    log::info!("In db out orders {:?}", orders);
    db::orders::transform_orders_database_changes(
        &mut tables,
        store_orders,
        store_nfts_item,
        orders,
        orders_executed,
        orders_cancelled,
    );

    Ok(tables.to_database_changes())
}

#[substreams::handlers::map]
fn db_out_polygon(
    collections: dcl::Collections,
    items: dcl::Items,
    nfts: dcl::NfTs,
    set_approved_events: dcl::CollectionSetApprovedEvents,
    set_store_minter_events: dcl::CollectionSetGlobalMinterEvents,
    set_item_minter_event: dcl::SetItemMinterEvents,
    orders: dcl::Orders,
    orders_executed: dcl::Orders,
    orders_cancelled: dcl::Orders,
    store_orders: StoreGetString,
    store_nfts_item: StoreGetString,
    store_collections_items_available: StoreGetString,
    store_items_mints: StoreGetInt64,
) -> Result<DatabaseChanges, substreams::errors::Error> {
    // let mut database_changes: DatabaseChanges = Default::default();
    let mut tables = substreams_database_change::tables::Tables::new();
    // Collections
    log::info!("In db out collections {:?}", collections);
    db::collections::transform_collection_database_changes(&mut tables, collections);
    // Items
    log::info!("In db out items {:?}", items);
    db::items::transform_item_v2_database_changes(&mut tables, items);
    // NFTs
    log::info!("In db out nfts {:?}", nfts);
    db::nfts::transform_nfts_database_changes(&mut tables, nfts.clone());
    // SetApprovedEvents
    log::info!("In db out set_events_approved {:?}", set_approved_events);
    db::collections::update_collection_is_approved(&mut tables, set_approved_events);
    // SetStoreMinterEvenbts
    log::info!("In db out set_store_minter {:?}", set_store_minter_events);
    db::collections::update_collection_search_is_store_minter(&mut tables, set_store_minter_events);
    log::info!(
        "In db out set_item_minter_event {:?}",
        set_item_minter_event
    );
    db::items::update_item_minter(&mut tables, set_item_minter_event);
    // get the item available count based on nfts minted
    db::items::update_item_available(
        &mut tables,
        nfts,
        store_items_mints,
        store_collections_items_available,
    );
    // Orders
    log::info!("In db out orders {:?}", orders);
    db::orders::transform_orders_database_changes(
        &mut tables,
        store_orders,
        store_nfts_item,
        orders,
        orders_executed,
        orders_cancelled,
    );

    Ok(tables.to_database_changes())
}
