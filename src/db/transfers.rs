use substreams_database_change::tables::Tables;

use crate::{dcl_hex, pb::dcl};

pub fn update_transfers(changes: &mut Tables, events: dcl::TransferEvents) {
    for event in events.events {
        let nft = format!("0x{}-{}", event.contract, event.token_id);
        changes
            .create_row("transfers", format!("{}-{}", nft, event.timestamp))
            .set("to", dcl_hex!(event.to))
            .set("from", dcl_hex!(event.from))
            .set("nft", nft)
            .set("timestamp", event.timestamp)
            .set("block_number", event.block_number);
    }
}
