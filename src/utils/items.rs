use substreams::log;

use crate::pb::dcl::Metadata;

use super::metadata::{build_emote_item, build_wearable_item};

pub const WEARABLE_V1: &str = "wearable_v1";
pub const WEARABLE_V2: &str = "wearable_v2";
pub const SMART_WEARABLE_V1: &str = "smart_wearable_v1";
pub const EMOTE_V1: &str = "emote_v1";
pub const WEARABLE_TYPE_SHORT: &str = "w";
pub const SMART_WEARABLE_TYPE_SHORT: &str = "sw";
pub const EMOTE_TYPE_SHORT: &str = "e";

pub struct ItemMetadata {
    pub item_type: String,
}

pub fn get_item_type_from_metadata(raw_metadata: String) -> ItemMetadata {
    let splitted: Vec<_> = raw_metadata.split(':').collect();
    log::info!("splitted {:?}", splitted);
    ItemMetadata {
        item_type: match splitted[1] {
            WEARABLE_TYPE_SHORT => WEARABLE_V2.to_string(),
            SMART_WEARABLE_TYPE_SHORT => SMART_WEARABLE_V1.to_string(),
            EMOTE_TYPE_SHORT => EMOTE_V1.to_string(),
            WEARABLE_V1 => WEARABLE_V1.to_string(),
            WEARABLE_V2 => WEARABLE_V2.to_string(),
            SMART_WEARABLE_V1 => SMART_WEARABLE_V1.to_string(),
            EMOTE_V1 => EMOTE_V1.to_string(),
            &_ => String::from(""), // fallback
        },
    }
}

pub fn build_metadata(item_id: &str, raw_metadata: &str, collection: &str) -> Metadata {
    let id = item_id.to_string();
    let mut metadata = Metadata {
        id,
        item_type: "".to_string(),
        wearable: None,
        emote: None,
    };

    let data: Vec<&str> = raw_metadata.split(':').collect();
    if data.len() >= 2 {
        let item_type = data[1];

        if item_type == WEARABLE_TYPE_SHORT {
            let wearable = build_wearable_item(item_id, raw_metadata, collection);
            if let Some(wearable) = wearable {
                metadata.item_type = WEARABLE_V2.to_string();
                metadata.wearable = Some(wearable);
            } else {
                metadata.item_type = "".to_string();
            }
        } else if item_type == SMART_WEARABLE_TYPE_SHORT {
            let wearable = build_wearable_item(item_id, raw_metadata, collection);
            if let Some(wearable) = wearable {
                metadata.item_type = SMART_WEARABLE_V1.to_string();
                metadata.wearable = Some(wearable);
            } else {
                metadata.item_type = "".to_string();
            }
        } else if item_type == EMOTE_TYPE_SHORT {
            let emote = build_emote_item(item_id, raw_metadata, collection);
            if let Some(emote) = emote {
                metadata.item_type = EMOTE_V1.to_string();
                metadata.emote = Some(emote);
            } else {
                metadata.item_type = "".to_string();
            }
        } else {
            metadata.item_type = "".to_string();
        }
    } else {
        metadata.item_type = "".to_string()
    }

    metadata
}

fn get_catalyst_base() -> String {
    let network = "mainnet"; //@TODO fix this
    if network == "mainnet" || network == "matic" {
        return "https://peer.decentraland.org".to_string();
    }

    if network == "mumbai" || network == "sepolia" {
        return "https://peer.decentraland.zone".to_string();
    }

    "".to_string()
}

pub fn get_item_image(item_urn: &str) -> String {
    let base_uri = get_catalyst_base();
    format!(
        "{}/lambdas/collections/contents/{}/thumbnail",
        base_uri, item_urn
    )
}

pub fn get_issued_id_from_token_uri(token_uri: &str) -> u32 {
    let splitted: Vec<&str> = token_uri.split('/').collect();

    // https://wearable-api.decentraland.org/v2/standards/erc721-metadata/collections/halloween_2019/wearables/funny_skull_mask/1
    // or
    // dcl://halloween_2019/vampire_feet/55
    if splitted.len() == 11 || splitted.len() == 5 {
        if let Some(issued_id) = splitted.last() {
            if let Ok(parsed_id) = issued_id.parse::<u32>() {
                return parsed_id;
            }
        }
    }

    0
}

pub fn get_wearable_id_from_token_uri(token_uri: &str) -> &str {
    let splitted: Vec<&str> = token_uri.split('/').collect();

    // https://wearable-api.decentraland.org/v2/standards/erc721-metadata/collections/halloween_2019/wearables/funny_skull_mask/1
    // or
    // dcl://halloween_2019/vampire_feet/55
    if splitted.len() == 11 || splitted.len() == 5 {
        let ids = &splitted[splitted.len() - 2..];
        return ids[0];
    }

    ""
}
