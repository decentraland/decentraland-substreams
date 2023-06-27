use crate::dcl_hex;
use crate::pb::dcl;
use substreams::prelude::BigInt;
use substreams_database_change::tables::Tables;

pub fn transform_nfts_database_changes(changes: &mut Tables, nfts: dcl::NfTs) {
    for nft in nfts.nfts {
        changes
            .create_row(
                "nfts",
                dcl_hex!(format!(
                    "{}-{}",
                    nft.collection_address,
                    nft.token_id.clone().unwrap().value
                )),
            )
            .set(
                "issued_id",
                BigInt::from(nft.issued_id.unwrap_or(dcl::BigInt {
                    value: String::from("0"),
                })),
            )
            .set("item", nft.item_id)
            .set("collection_id", dcl_hex!(nft.collection_address))
            .set(
                "token_id",
                BigInt::from(nft.token_id.unwrap_or(dcl::BigInt {
                    value: String::from("0"),
                })),
            )
            .set("created_at", nft.created_at)
            .set("updated_at", nft.updated_at)
            .set("owner", dcl_hex!(nft.beneficiary));
    }
}
