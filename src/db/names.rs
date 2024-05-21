use substreams_database_change::tables::Tables;

use crate::{dcl_hex, pb::dcl};

pub fn transform_name_transfers_database_changes(
    changes: &mut Tables,
    name_transfers: dcl::NameTransfers,
) {
    for transfer in &name_transfers.name_transfers {
        changes
            .create_row(
                "name_transfers",
                format!("{}-{}", transfer.token_id, transfer.timestamp),
            )
            .set("from", dcl_hex!(transfer.from))
            .set("to", dcl_hex!(transfer.to))
            .set("token_id", &transfer.token_id)
            .set("timestamp", transfer.timestamp)
            .set("block_number", transfer.block_number);
    }
}
