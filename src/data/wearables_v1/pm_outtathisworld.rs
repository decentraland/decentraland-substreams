use super::wearable::Wearable;

pub const pm_outtathisworld: Vec<Wearable> = vec![
    Wearable {
        id: String::from("pm_col1_alien_helmet "),
        name: String::from("Alien Helmet "),
        description: String::from("An out of this world hat for the believers "),
        category: String::from("helmet "),
        rarity: String::from("legendary "),
        body_shapes: vec![String::from("BaseMale"), String::from("BaseFemale")],
    },
    Wearable {
        id: String::from("pm_col1_cat_helmet "),
        name: String::from("Cat Helmet "),
        description: String::from("A domesticated purring mammal helmet for Cat Lovers "),
        category: String::from("helmet "),
        rarity: String::from("legendary "),
        body_shapes: vec![String::from("BaseMale"), String::from("BaseFemale")],
    },
    Wearable {
        id: String::from("pm_col1_dino_helmet "),
        name: String::from("Dino Helmet "),
        description: String::from("A cartoon pre-human predator helmet "),
        category: String::from("helmet "),
        rarity: String::from("legendary "),
        body_shapes: vec![String::from("BaseMale"), String::from("BaseFemale")],
    },
    Wearable {
        id: String::from("pm_col1_panda_helmet "),
        name: String::from("Panda Helmet "),
        description: String::from("A bamboo-eater helmet for Panda Lovers "),
        category: String::from("helmet "),
        rarity: String::from("legendary "),
        body_shapes: vec![String::from("BaseMale"), String::from("BaseFemale")],
    },
    Wearable {
        id: String::from("pm_col1_cargo_pants_lower_body "),
        name: String::from("Polygonal Mind - Cargo Pants "),
        description: String::from("Some trendy triangle-patterned pants for you to show off "),
        category: String::from("lower_body "),
        rarity: String::from("legendary "),
        body_shapes: vec![String::from("BaseMale"), String::from("BaseFemale")],
    },
    Wearable {
        id: String::from("pm_col1_trendy_coat_upper_body "),
        name: String::from("Polygonal Mind - Trendy Coat "),
        description: String::from(
            "A trendy triangle-patterned coat for you to show off and stay warm in winter ",
        ),
        category: String::from("upper_body "),
        rarity: String::from("legendary "),
        body_shapes: vec![String::from("BaseMale"), String::from("BaseFemale")],
    },
];
