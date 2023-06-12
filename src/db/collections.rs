use crate::dcl_hex;
use crate::pb::dcl;
use crate::utils::sanitize_sql_string;
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
            .change("creator", (None, dcl_hex!(collection.creator)))
            .change("is_approved", (None, collection.is_approved))
            .change("is_completed", (None, collection.is_completed))
            .change("name", (None, sanitize_sql_string(collection.name)))
            .change("symbol", (None, sanitize_sql_string(collection.symbol)))
            .change("owner", (None, dcl_hex!(collection.owner)))
            .change("created_at", (None, collection.created_at));
    }
}
