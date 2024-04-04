use crate::dcl_hex;
use crate::pb::dcl;
use crate::utils::sanitize_sql_string;
use crate::utils::store::get_store_address;
use substreams_database_change::tables::Tables;

pub fn transform_collection_database_changes(changes: &mut Tables, collections: dcl::Collections) {
    for collection in collections.collections {
        changes
            .create_row("collections", dcl_hex!(collection.address.clone()))
            .set("owner", dcl_hex!(collection.owner))
            .set("creator", dcl_hex!(collection.creator))
            .set("name", sanitize_sql_string(collection.name))
            .set("symbol", sanitize_sql_string(collection.symbol))
            .set("is_completed", collection.is_completed)
            .set("is_editable", collection.is_editable)
            .set("urn", collection.urn)
            // .set("managers", managers2) @TODO: fix this and add minters as well
            .set("created_at", collection.created_at)
            .set("updated_at", collection.updated_at)
            .set("reviewed_at", collection.reviewed_at)
            .set(
                "first_listed_at",
                collection.first_listed_at.unwrap_or_default(),
            )
            .set("block_number", collection.block_number);
    }
}

pub fn insert_collection_is_approved_event(
    changes: &mut Tables,
    set_approved_events: dcl::CollectionSetApprovedEvents,
) {
    for event in set_approved_events.events {
        changes
            .create_row(
                "collection_set_approved_events",
                format!("{}{}", dcl_hex!(event.collection.clone()), event.updated_at),
            )
            .set("collection_id", dcl_hex!(event.collection.clone()))
            .set("value", event.new_value)
            .set("timestamp", event.updated_at)
            .set("block_number", event.block_number);
    }
}

pub fn update_collection_transfer_ownership_event(
    changes: &mut Tables,
    transfer_ownership_events: dcl::CollectionTransferOwnershipEvents,
) {
    for event in transfer_ownership_events.events {
        changes
            .update_row("collections", dcl_hex!(event.collection))
            .set("owner", dcl_hex!(event.to));
    }
}

pub fn update_collection_transfer_creatoriship_event(
    changes: &mut Tables,
    transfer_creatorship_events: dcl::CollectionTransferCreatorshipEvents,
) {
    for event in transfer_creatorship_events.events {
        changes
            .update_row("collections", dcl_hex!(event.collection.clone()))
            .set("creator", dcl_hex!(event.to.clone()));
    }
}

pub fn insert_collection_search_is_store_minter(
    network: &str,
    changes: &mut Tables,
    events: dcl::CollectionSetGlobalMinterEvents,
) {
    for event in events.events {
        let is_store_minter = event.value && dcl_hex!(event.minter) == get_store_address(network);
        changes
            .create_row(
                "collection_set_global_minter_events",
                format!("{}-{}", dcl_hex!(event.collection.clone()), event.timestamp),
            )
            .set("collection_id", dcl_hex!(event.collection.clone()))
            .set("minter", dcl_hex!(event.minter))
            .set("value", event.value)
            .set("search_is_store_minter", is_store_minter)
            .set("timestamp", event.timestamp)
            .set("block_number", event.block_number);
    }
}
