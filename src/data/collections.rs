// use wearables_v1::*;

// use super::atari_launch;

// mod wearables_v1;

use super::wearables_v1::*;

const all_collections: Vec<Vec<wearable::Wearable>> = vec![
    atari_launch::atari_launch,
    // binance_us_collection.to_vec(),
    // china_flying.to_vec(),
    // community_contest.to_vec(),
    // wearable_test.to_vec(),
];

pub fn find_wearable<'a>(
    wearable_id: &str,
    // collection: &'a [wearable::Wearable],
) -> Option<&'a wearable::Wearable> {
    for collection in all_collections {
        for wearable in collection {
            if wearable.id == wearable_id {
                return Some(&wearable);
            }
        }
    }
    None
}
