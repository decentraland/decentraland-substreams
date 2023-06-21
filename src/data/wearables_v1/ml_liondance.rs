use super::wearable::Wearable;

pub const ml_liondance: Vec<Wearable> = vec![
    Wearable {
        id: String::from("lion_dance_feet "),
        name: String::from("Lion Dance Shoes "),
        description: String::from("Dragon City 2021 Spring Festival Souvenirs "),
        category: String::from("feet "),
        rarity: String::from("epic "),
        body_shapes: vec![String::from("BaseMale"), String::from("BaseFemale")],
    },
    Wearable {
        id: String::from("lion_dance_hat "),
        name: String::from("Lion Dance Hat "),
        description: String::from("Dragon City 2021 Spring Festival Souvenirs "),
        category: String::from("hat "),
        rarity: String::from("epic "),
        body_shapes: vec![String::from("BaseMale"), String::from("BaseFemale")],
    },
    Wearable {
        id: String::from("lion_dance_lower_body "),
        name: String::from("Lion Dance Pants "),
        description: String::from("Dragon City 2021 Spring Festival Souvenirs "),
        category: String::from("lower_body "),
        rarity: String::from("epic "),
        body_shapes: vec![String::from("BaseMale"), String::from("BaseFemale")],
    },
    Wearable {
        id: String::from("lion_dance_upper_body "),
        name: String::from("Lion Dance Coat "),
        description: String::from("Dragon City 2021 Spring Festival Souvenirs "),
        category: String::from("upper_body "),
        rarity: String::from("epic "),
        body_shapes: vec![String::from("BaseMale"), String::from("BaseFemale")],
    },
];
