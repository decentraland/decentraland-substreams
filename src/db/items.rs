use crate::pb::dcl;
use crate::utils::sanitize_sql_string;
use crate::{dcl_hex, utils::format_postgres_array};
use substreams::prelude::BigInt;
use substreams::store::{StoreGet, StoreGetInt64};
use substreams_database_change::tables::Tables;

fn get_item_changes(changes: &mut Tables, item: dcl::Item, blockchain_id: Option<i64>) {
    let sanitized_metadata = sanitize_sql_string(item.raw_metadata);
    let metadata = item.metadata.unwrap();
    changes
        .create_row("items", item.id.clone())
        .set("collection", dcl_hex!(item.collection))
        .set("blockchain_id", BigInt::from(blockchain_id.unwrap_or(0)))
        .set("creator", dcl_hex!(item.creator))
        .set("item_type", item.item_type)
        .set(
            "total_supply",
            BigInt::from(item.total_supply.clone().unwrap_or(dcl::BigInt {
                value: String::from("0"),
            })),
        )
        .set(
            "max_supply",
            BigInt::from(item.max_supply.clone().unwrap_or(dcl::BigInt {
                value: String::from("0"),
            })),
        )
        .set("rarity", item.rarity)
        .set(
            "creation_fee",
            BigInt::from(item.creation_fee.unwrap_or(dcl::BigInt {
                value: String::from("0"),
            })),
        )
        .set(
            "available",
            BigInt::from(item.max_supply.unwrap_or(dcl::BigInt {
                value: String::from("0"),
            })),
        )
        .set(
            "price",
            BigInt::from(item.price.unwrap_or(dcl::BigInt {
                value: String::from("0"),
            })),
        )
        .set("beneficiary", dcl_hex!(item.beneficiary))
        .set(
            "content_hash",
            item.content_hash.unwrap_or_else(|| String::from("")),
        )
        .set("uri", item.uri)
        .set("image", item.image)
        .set("raw_metadata", sanitized_metadata)
        .set("metadata", metadata.id.clone())
        .set("urn", item.urn)
        .set("created_at", item.created_at)
        .set("reviewed_at", item.reviewed_at)
        .set("updated_at", item.updated_at)
        .set("sold_at", item.sold_at.unwrap_or(0))
        .set("first_listed_at", item.first_listed_at.unwrap_or(0))
        .set("block_number", item.block_number);

    // metadata wearable change
    if let Some(wearable) = metadata.wearable {
        changes
            .create_row("metadata", metadata.id.clone())
            .set("item_type", metadata.item_type.clone())
            .set("wearable", wearable.id.clone());

        changes
            .create_row("wearable", wearable.id)
            .set("name", wearable.name)
            .set("description", wearable.description)
            .set("collection", wearable.collection)
            .set("category", wearable.category)
            .set("rarity", wearable.rarity)
            .set("body_shapes", format_postgres_array(wearable.body_shapes));
    }
    // metadata emote change
    if let Some(emote) = metadata.emote {
        changes
            .create_row("metadata", metadata.id)
            .set("item_type", metadata.item_type)
            .set("emote", emote.id.clone());

        changes
            .create_row("emote", emote.id)
            .set("name", emote.name)
            .set("description", emote.description)
            .set("collection", emote.collection)
            .set("category", emote.category)
            .set("rarity", emote.rarity)
            .set("loop", emote.r#loop)
            .set("body_shapes", format_postgres_array(emote.body_shapes));
    };
}

pub fn transform_item_v1_database_changes(
    changes: &mut Tables,
    items: dcl::Items,
    collections_v1_store: StoreGetInt64,
) {
    for item in items.items {
        let blockchain_id = collections_v1_store.get_last(item.collection.clone());
        get_item_changes(changes, item, blockchain_id);
    }
}

pub fn transform_item_v2_database_changes(changes: &mut Tables, items: dcl::Items) {
    for item in items.items {
        let blockchain_id = Some(item.blockchain_id);
        get_item_changes(changes, item, blockchain_id);
    }
}

pub fn transform_item_database_changes(
    network: String,
    changes: &mut Tables,
    items: dcl::Items,
    collections_v1_store: StoreGetInt64,
) {
    if network == "polygon" {
        transform_item_v2_database_changes(changes, items);
    } else {
        transform_item_v1_database_changes(changes, items, collections_v1_store);
    }
}

pub fn update_item_minter(changes: &mut Tables, events: dcl::SetItemMinterEvents) {
    for event in events.events {
        let item = format!("0x{}-{}", event.collection, event.item);
        changes
            .create_row("item_minters", format!("{}-{}", item, event.timestamp))
            .set("item_id", item)
            .set("minter", dcl_hex!(event.minter))
            .set("value", event.value.parse::<i64>().unwrap() > 0)
            .set("timestamp", event.timestamp)
            .set("block_number", event.block_number);
    }
}
