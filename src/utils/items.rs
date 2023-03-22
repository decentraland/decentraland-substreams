use substreams::log;

const WEARABLE: &str = "wearable";
const EMOTE: &str = "emote";
const SMART_WEARABLE: &str = "smart_wearable";

const WEARABLE_V1: &str = "wearable_v1";
const WEARABLE_V2: &str = "wearable_v2";
const SMART_WEARABLE_V1: &str = "smart_wearable_v1";
const EMOTE_V1: &str = "emote_v1";
const WEARABLE_TYPE_SHORT: &str = "w";
const SMART_WEARABLE_TYPE_SHORT: &str = "sw";
const EMOTE_TYPE_SHORT: &str = "e";

pub struct ItemMetadata {
    pub item_type: String,
}

pub fn build_item_metadata(raw_metadata: String) -> ItemMetadata {
    let splitted: Vec<_> = raw_metadata.split(':').collect();
    log::info!("splitted {:?}", splitted);
    ItemMetadata {
        item_type: match splitted[1] {
            WEARABLE_V1 => WEARABLE.to_string(),
            WEARABLE_V2 => WEARABLE.to_string(),
            WEARABLE_TYPE_SHORT => WEARABLE.to_string(),
            SMART_WEARABLE_V1 => SMART_WEARABLE.to_string(),
            SMART_WEARABLE_TYPE_SHORT => SMART_WEARABLE.to_string(),
            EMOTE_TYPE_SHORT => EMOTE.to_string(),
            EMOTE_V1 => EMOTE.to_string(),
            &_ => String::from(""), // fallback
        },
    }
}
