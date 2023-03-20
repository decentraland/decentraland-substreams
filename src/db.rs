use crate::dcl_hex;
use crate::pb::dcl;
use crate::utils::sanitize_sql_string;
use substreams::prelude::BigInt;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};

pub fn transform_collection_database_changes(
    changes: &mut DatabaseChanges,
    collections: dcl::Collections,
) {
    for collection in collections.collections {
        changes
            .push_change(
                String::from("collections"),
                dcl_hex!(collection.address.clone()),
                0,
                table_change::Operation::Create,
            )
            .change("created_at", (None, collection.created_at))
            .change("creator", (None, collection.creator));
    }
}

pub fn transform_item_database_changes(changes: &mut DatabaseChanges, items: dcl::Items) {
    for item in items.items {
        let sanitized_metadata = sanitize_sql_string(item.metadata);
        changes
            .push_change(
                "items".to_string(),
                item.id.clone(),
                0,
                table_change::Operation::Create,
            )
            .change(
                "blockchain_item_id",
                (None, BigInt::from(item.blockchain_item_id.unwrap())),
            )
            .change("creator", (None, String::from("some creator"))) //@TODO get this value, we could use the one from collections
            .change("item_type", (None, String::from("wearable"))) //@TODO get this value, parse from metadata
            .change("price", (None, BigInt::from(item.price.unwrap())))
            .change(
                "total_supply",
                (None, BigInt::from(item.total_supply.clone().unwrap())),
            )
            .change(
                "available",
                (None, BigInt::from(item.total_supply.unwrap())),
            )
            .change("max_supply", (None, BigInt::from(item.max_supply.unwrap())))
            .change("rarity", (None, item.rarity))
            .change("beneficiary", (None, item.beneficiary))
            .change("raw_metadata", (None, sanitized_metadata))
            .change("created_at", (None, item.created_at))
            .change("collection_id", (None, item.collection_id));
    }
}
