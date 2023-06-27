use super::wearable::Wearable;

pub fn tech_tribal_marc0matic() -> Vec<Wearable> {
    vec![
    Wearable {
        id: String::from("techtribal_shoes"),
        name: String::from("Tech Tribal Shoes"),
        description: String::from("Tech Tribal Leather Band Shoes."),
        category: String::from("feet"),
        rarity: String::from("legendary"),
        body_shapes: vec![String::from("BaseMale"), String::from("BaseFemale")],
    },
    Wearable {
        id: String::from("tech_tribal_trousers"),
        name: String::from("Tech Tribal Trousers"),
        description: String::from("Tech Tribal Trousers with armored Leg Braces."),
        category: String::from("lower_body"),
        rarity: String::from("legendary"),
        body_shapes: vec![String::from("BaseMale"), String::from("BaseFemale")],
    },
    Wearable {
        id: String::from("techtribal_beast_mask"),
        name: String::from("Tech Tribal Beast Mask"),
        description: String::from("VR Infused Beast Mask created by Marc0Matic"),
        category: String::from("mask"),
        rarity: String::from("legendary"),
        body_shapes: vec![String::from("BaseMale"), String::from("BaseFemale")],
    },
    Wearable {
        id: String::from("techtribal_bird_mask"),
        name: String::from("Tech Tribal Bird Mask"),
        description: String::from("VR Infused Bird Mask created by Marc0Matic"),
        category: String::from("mask"),
        rarity: String::from("legendary"),
        body_shapes: vec![String::from("BaseMale"), String::from("BaseFemale")],
    },
    Wearable {
        id: String::from("techtribal_shaman_garb"),
        name: String::from("Tech Tribal Shaman Garb"),
        description: String::from("Tech Tribal Shaman Garb. Worn by those familiar with 'Machine Speak.'"),
        category: String::from("upper_body"),
        rarity: String::from("legendary"),
        body_shapes: vec![String::from("BaseMale"), String::from("BaseFemale")],
    },
    Wearable {
        id: String::from("techtribal_solar_garb"),
        name: String::from("Tech Tribal Solar Garb"),
        description: String::from("Tech Tribal Solar Garb with Solar Shoulder Pads. Worn by Mystics fluent in 'Machine Speak."),
        category: String::from("upper_body"),
        rarity: String::from("legendary"),
        body_shapes: vec![String::from("BaseMale"), String::from("BaseFemale")],
    },
]
}
