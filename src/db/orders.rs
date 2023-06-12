use crate::pb::dcl;
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
