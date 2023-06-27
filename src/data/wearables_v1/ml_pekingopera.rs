use super::wearable::Wearable;

pub fn ml_pekingopera() -> Vec<Wearable> {
    vec![
        Wearable {
            id: String::from("jing_feet"),
            name: String::from("Peking Opera Jing Shoes"),
            description: String::from("Peking Opera Character-Jing"),
            category: String::from("feet"),
            rarity: String::from("legendary"),
            body_shapes: vec![String::from("BaseMale"), String::from("BaseFemale")],
        },
        Wearable {
            id: String::from("jing_hat"),
            name: String::from("Peking Opera Jing Hat"),
            description: String::from("Peking Opera Character-Jing"),
            category: String::from("hat"),
            rarity: String::from("legendary"),
            body_shapes: vec![String::from("BaseMale"), String::from("BaseFemale")],
        },
        Wearable {
            id: String::from("jing_lower_body"),
            name: String::from("Peking Opera Jing Pants"),
            description: String::from("Peking Opera Character-Jing"),
            category: String::from("lower_body"),
            rarity: String::from("legendary"),
            body_shapes: vec![String::from("BaseMale"), String::from("BaseFemale")],
        },
        Wearable {
            id: String::from("jing_xiangyu_mask"),
            name: String::from("Peking Opera Xiang Yu Mask"),
            description: String::from("Peking Opera Character-Xiang Yu"),
            category: String::from("mask"),
            rarity: String::from("legendary"),
            body_shapes: vec![String::from("BaseMale"), String::from("BaseFemale")],
        },
        Wearable {
            id: String::from("jing_yingbu_mask"),
            name: String::from("Peking Opera Ying Bu Mask"),
            description: String::from("Peking Opera Character-Ying Bu"),
            category: String::from("mask"),
            rarity: String::from("legendary"),
            body_shapes: vec![String::from("BaseMale"), String::from("BaseFemale")],
        },
        Wearable {
            id: String::from("jing_upper_body"),
            name: String::from("Peking Opera Jing Coat"),
            description: String::from("Peking Opera Character-Jing"),
            category: String::from("upper_body"),
            rarity: String::from("legendary"),
            body_shapes: vec![String::from("BaseMale"), String::from("BaseFemale")],
        },
    ]
}
