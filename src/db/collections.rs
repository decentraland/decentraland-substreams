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
            .set("is_approved", collection.is_approved)
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
            .set("search_is_store_minter", collection.search_is_store_minter);
    }
}

pub fn update_collection_is_approved(
    changes: &mut Tables,
    set_approved_events: dcl::CollectionSetApprovedEvents,
) {
    for event in set_approved_events.events {
        changes
            .update_row("collections", dcl_hex!(event.collection.clone()))
            .set("is_approved", event.new_value)
            .set("updated_at", event.updated_at.clone())
            .set("reviewed_at", event.updated_at);
    }
}

pub fn update_collection_search_is_store_minter(
    changes: &mut Tables,
    events: dcl::CollectionSetGlobalMinterEvents,
) {
    for event in events.events {
        let is_store_minter = event.value && event.minter == get_store_address("matic");
        changes
            .update_row("collections", dcl_hex!(event.collection.clone()))
            .set("search_is_store_minter", is_store_minter);
    }
}
