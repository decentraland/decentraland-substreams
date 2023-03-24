use crate::pb::dcl;
use crate::utils::sanitize_sql_string;
use crate::{dcl_hex, pb::dcl::Collections};
use substreams::prelude::BigInt;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};

pub fn transform_collection_database_changes(
    changes: &mut DatabaseChanges,
    collections: Collections,
) {
    for collection in collections.collections {
        changes
            .push_change(
                String::from("collections"),
                dcl_hex!(collection.address.clone()),
                0,
                table_change::Operation::Create,
            )
            .change("creator", (None, dcl_hex!(collection.creator)))
            .change("is_approved", (None, collection.is_approved))
            .change("is_completed", (None, collection.is_completed))
            .change("name", (None, sanitize_sql_string(collection.name)))
            .change("symbol", (None, collection.symbol))
            .change("owner", (None, dcl_hex!(collection.owner)))
            .change("created_at", (None, collection.created_at));
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
                (
                    None,
                    BigInt::from(item.blockchain_item_id.unwrap_or(dcl::BigInt {
                        value: String::from("0"),
                    })),
                ),
            )
            .change("item_type", (None, item.item_type))
            .change(
                "price",
                (
                    None,
                    BigInt::from(item.price.unwrap_or(dcl::BigInt {
                        value: String::from("0"),
                    })),
                ),
            )
            .change(
                "total_supply",
                (
                    None,
                    BigInt::from(item.total_supply.clone().unwrap_or(dcl::BigInt {
                        value: String::from("0"),
                    })),
                ),
            )
            .change(
                "available",
                (
                    None,
                    BigInt::from(item.max_supply.clone().unwrap_or(dcl::BigInt {
                        value: String::from("0"),
                    })),
                ),
            )
            .change(
                "max_supply",
                (
                    None,
                    BigInt::from(item.max_supply.unwrap_or(dcl::BigInt {
                        value: String::from("0"),
                    })),
                ),
            )
            .change("rarity", (None, item.rarity))
            .change("beneficiary", (None, item.beneficiary))
            .change("raw_metadata", (None, sanitized_metadata))
            .change("created_at", (None, item.created_at))
            .change("collection_id", (None, item.collection_id));
    }
}
