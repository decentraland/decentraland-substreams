use crate::dcl_hex;
use crate::pb::dcl;
use substreams::prelude::BigInt;
use substreams::store::StoreGet;
use substreams_database_change::tables::Tables;

pub fn transform_orders_status_database_changes(changes: &mut Tables, orders: dcl::Orders) {
    for order in orders.orders {
        changes
            .update_row("orders", dcl_hex!(order.id.clone()))
            .set(
                "price",
                BigInt::from(order.price.unwrap_or(dcl::BigInt {
                    value: String::from("0"),
                })),
            )
            .set("buyer", dcl_hex!(order.buyer))
            .set("status", order.status)
            .set("block_number", order.block_number)
            .set("updated_at", order.updated_at);
    }
}

pub fn transform_orders_created_database_changes(
    changes: &mut Tables,
    orders: dcl::Orders,
    nfts_item_store: substreams::store::StoreGetString,
) {
    for order in orders.orders {
        substreams::log::info!(
            "transform_orders_created_database_changes2 {:?}",
            order.nft.clone()
        );
        match nfts_item_store.get_last(order.nft.clone()) {
            Some(item) => {
                substreams::log::info!(
                    "transform_orders_created_database_changes2 match! {:?}",
                    item
                );
                changes
                    .create_row("orders", dcl_hex!(order.id.clone()))
                    .set("marketplace_address", dcl_hex!(order.marketplace_address))
                    .set("item", item)
                    .set("nft", dcl_hex!(order.nft))
                    .set("nft_address", order.nft_address)
                    .set(
                        "token_id",
                        BigInt::from(order.token_id.unwrap_or(dcl::BigInt {
                            value: String::from("0"),
                        })),
                    )
                    .set("tx_hash", dcl_hex!(order.tx_hash))
                    .set("owner", dcl_hex!(order.owner))
                    .set("price", order.price.unwrap().value)
                    .set("status", order.status)
                    .set("block_number", order.block_number)
                    .set(
                        "expires_at",
                        BigInt::from(order.expires_at.unwrap_or(dcl::BigInt {
                            value: String::from("0"),
                        })),
                    )
                    .set("created_at", order.created_at)
                    .set("updated_at", order.updated_at);
            }
            None => continue,
        }
    }
}

pub fn transform_orders_database_changes(
    changes: &mut Tables,
    nfts_item_store: substreams::store::StoreGetString,
    orders_created: dcl::Orders,
    orders_executed: dcl::Orders,
    orders_cancelled: dcl::Orders,
) {
    transform_orders_created_database_changes(changes, orders_created, nfts_item_store);
    substreams::log::info!("In db out orders_executed {:?}", orders_executed);
    transform_orders_status_database_changes(changes, orders_executed);
    substreams::log::info!("In db out orders_cancelled {:?}", orders_cancelled);
    transform_orders_status_database_changes(changes, orders_cancelled);
}
