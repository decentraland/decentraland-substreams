use crate::pb::dcl::{Emote, Item, Wearable};

pub fn build_wearable_item(item: &Item) -> Option<Wearable> {
    let id = item.id.clone();
    let data: Vec<&str> = item.raw_metadata.split(':').collect();

    if (data.len() == 6 || data.len() == 8)
        && is_valid_wearable_category(data[4])
        && is_valid_body_shape(&data[5].split(',').collect::<Vec<&str>>()[..])
    {
        let wearable = Wearable {
            id: id.clone(),
            name: data[2].to_string(),
            description: data[3].to_string(),
            collection: item.collection.clone(),
            category: data[4].to_string(),
            rarity: item.rarity.to_string(),
            body_shapes: data[5]
                .split(',')
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
        };

        return Some(wearable);
    }

    None
}

fn is_valid_wearable_category(category: &str) -> bool {
    match category {
        "eyebrows" | "eyes" | "facial_hair" | "hair" | "mouth" | "upper_body" | "lower_body"
        | "feet" | "earring" | "eyewear" | "hat" | "helmet" | "mask" | "tiara" | "top_head"
        | "skin" => true,
        _ => {
            substreams::log::info!("Invalid Category {}", category);
            false
        }
    }
}

fn is_valid_body_shape(body_shapes: &[&str]) -> bool {
    for body_shape in body_shapes {
        if *body_shape != "BaseFemale" && *body_shape != "BaseMale" {
            substreams::log::info!("Invalid BodyShape {}", body_shape);
            return false;
        }
    }
    true
}

// Emotes

pub fn build_emote_item(item: &Item) -> Option<Emote> {
    let id = item.id.clone();
    let data: Vec<&str> = item.raw_metadata.split(':').collect();
    let data_has_valid_length = data.len() == 6 || data.len() == 7 || data.len() == 8;

    if data_has_valid_length && is_valid_body_shape(&data[5].split(',').collect::<Vec<&str>>()[..])
    {
        let emote = Emote {
            id: id.clone(),
            collection: item.collection.clone(),
            name: data[2].to_string(),
            description: data[3].to_string(),
            rarity: item.rarity.to_string(),
            category: if is_valid_emote_category(data[4]) {
                data[4].to_string()
            } else {
                "DANCE".to_string() // Fallback to "DANCE" for old emotes
            },
            body_shapes: data[5]
                .split(',')
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
            r#loop: data.len() == 7 && is_valid_loop_value(data[6]) && data[6] == "1",
        };

        return Some(emote);
    }

    None
}

fn is_valid_emote_category(category: &str) -> bool {
    match category {
        "DANCE" | "STUNT" | "GREETINGS" | "FUN" | "POSES" | "REACTIONS" | "HORROR"
        | "MISCELLANEOUS" => true,
        _ => {
            substreams::log::info!("Invalid Category {}", category);
            false
        }
    }
}

fn is_valid_loop_value(value: &str) -> bool {
    match value {
        "0" | "1" => true,
        _ => {
            substreams::log::info!("Invalid emote loop value {}", value);
            false
        }
    }
}
