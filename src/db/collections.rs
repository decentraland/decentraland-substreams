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
            .change("owner", (None, dcl_hex!(collection.owner)))
            .change("creator", (None, dcl_hex!(collection.creator)))
            .change("name", (None, sanitize_sql_string(collection.name)))
            .change("symbol", (None, sanitize_sql_string(collection.symbol)))
            .change("is_completed", (None, collection.is_completed))
            .change("is_approved", (None, collection.is_approved))
            .change("is_editable", (None, collection.is_editable))
            .change("urn", (None, collection.urn))
            // .change(
            //     "managers",
            //     (
            //         None,
            //         collection
            //             .managers
            //             .into_iter()
            //             .flat_map(|s| s.into_bytes())
            //             .collect::<Vec<u8>>(),
            //     ),
            // )
            // .change(
            //     "minters",
            //     (
            //         None,
            //         collection
            //             .minters
            //             .into_iter()
            //             .flat_map(|s| s.into_bytes())
            //             .collect::<Vec<u8>>(),
            //     ),
            // )
            .change("created_at", (None, collection.created_at))
            .change("reviewed_at", (None, collection.reviewed_at))
            .change(
                "first_listed_at",
                (None, collection.first_listed_at.unwrap_or_default()),
            )
            .change(
                "search_is_store_minter",
                (None, collection.search_is_store_minter),
            );
    }
}
