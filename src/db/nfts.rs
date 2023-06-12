use crate::dcl_hex;
use crate::pb::dcl;
use substreams::prelude::BigInt;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};

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
