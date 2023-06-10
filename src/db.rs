use crate::pb::dcl;
use crate::utils::sanitize_sql_string;
use crate::{dcl_hex, utils};
use substreams::prelude::BigInt;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};

pub fn cancel_nft_order(changes: &mut DatabaseChanges, order_id: String) {
    changes
        .push_change(
            String::from("orders"),
            dcl_hex!(order_id),
            0,
            table_change::Operation::Update,
        )
        .change("status", (None, utils::orders::ORDER_CANCELLED));
}

pub fn transform_orders_status_database_changes(
    changes: &mut DatabaseChanges,
    orders: dcl::Orders,
) {
    for order in orders.orders {
        changes
            .push_change(
                String::from("orders"),
                dcl_hex!(order.id.clone()),
                0,
                table_change::Operation::Update,
            )
            .change(
                "price",
                (
                    None,
                    BigInt::from(order.price.unwrap_or(dcl::BigInt {
                        value: String::from("0"),
                    })),
                ),
            )
            .change("buyer", (None, dcl_hex!(order.buyer)))
            .change("status", (None, order.status))
            .change("block_number", (None, order.block_number))
            .change("updated_at", (None, order.updated_at));
    }
}

pub fn transform_orders_database_changes(changes: &mut DatabaseChanges, orders: dcl::Orders) {
    for order in orders.orders {
        changes
            .push_change(
                String::from("orders"),
                dcl_hex!(order.id.clone()),
                0,
                table_change::Operation::Create,
            )
            .change(
                "marketplace_address",
                (None, dcl_hex!(order.marketplace_address)),
            )
            .change("nft_id", (None, dcl_hex!(order.nft_id)))
            .change(
                "token_id",
                (
                    None,
                    BigInt::from(order.token_id.unwrap_or(dcl::BigInt {
                        value: String::from("0"),
                    })),
                ),
            )
            .change("tx_hash", (None, dcl_hex!(order.tx_hash)))
            .change("owner", (None, dcl_hex!(order.owner)))
            .change("price", (None, order.price.unwrap().value))
            .change("status", (None, order.status))
            .change("block_number", (None, order.block_number))
            .change(
                "expires_at",
                (
                    None,
                    BigInt::from(order.expires_at.unwrap_or(dcl::BigInt {
                        value: String::from("0"),
                    })),
                ),
            )
            .change("updated_at", (None, order.updated_at));
    }
}

pub fn transform_nfts_database_changes(changes: &mut DatabaseChanges, nfts: dcl::NfTs) {
    for nft in nfts.nfts {
        changes
            .push_change(
                String::from("nfts"),
                dcl_hex!(format!(
                    "{}-{}",
                    nft.collection_address,
                    nft.token_id.clone().unwrap().value
                )),
                0,
                table_change::Operation::Create,
            )
            .change(
                "issued_id",
                (
                    None,
                    BigInt::from(nft.issued_id.unwrap_or(dcl::BigInt {
                        value: String::from("0"),
                    })),
                ),
            )
            .change("item_id", (None, nft.item_id))
            .change("collection_id", (None, dcl_hex!(nft.collection_address)))
            .change(
                "token_id",
                (
                    None,
                    BigInt::from(nft.token_id.unwrap_or(dcl::BigInt {
                        value: String::from("0"),
                    })),
                ),
            )
            .change("created_at", (None, nft.created_at))
            .change("updated_at", (None, nft.updated_at))
            .change("owner", (None, dcl_hex!(nft.beneficiary)));
    }
}

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
            .change("creator", (None, dcl_hex!(collection.creator)))
            .change("is_approved", (None, collection.is_approved))
            .change("is_completed", (None, collection.is_completed))
            .change("name", (None, sanitize_sql_string(collection.name)))
            .change("symbol", (None, sanitize_sql_string(collection.symbol)))
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
            .change("collection_id", (None, dcl_hex!(item.collection_id)));
    }
}

pub fn transform_transfers_database_changes(
    changes: &mut DatabaseChanges,
    transfers: dcl::Transfers,
) {
    for transfer in transfers.transfers {
        changes
            .push_change(
                String::from("transfers"),
                dcl_hex!(format!(
                    "{}-{}",
                    transfer.tx_hash.clone(),
                    transfer.log_index
                )),
                0,
                table_change::Operation::Create,
            )
            .change("collection_id", (None, dcl_hex!(transfer.collection_id)))
            .change(
                "token_id",
                (
                    None,
                    BigInt::from(transfer.token_id.unwrap_or(dcl::BigInt {
                        value: String::from("0"),
                    })),
                ),
            )
            .change("block_timestamp", (None, transfer.block_timestamp))
            .change("from_address", (None, dcl_hex!(transfer.from)))
            .change("to_address", (None, dcl_hex!(transfer.to)));
    }
}
