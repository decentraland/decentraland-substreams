use super::wearable::Wearable;

pub fn release_the_kraken() -> Vec<Wearable> {
    vec![
        Wearable {
            id: String::from("rac_feet"),
            name: String::from("RAC footwear"),
            description: String::from("RAC's signature pink footwear."),
            category: String::from("feet"),
            rarity: String::from("unique"),
            body_shapes: vec![String::from("BaseMale"), String::from("BaseFemale")],
        },
        Wearable {
            id: String::from("rac_hat"),
            name: String::from("RAC Cap"),
            description: String::from("RAC's signature pink baseball cap."),
            category: String::from("hat"),
            rarity: String::from("unique"),
            body_shapes: vec![String::from("BaseMale"), String::from("BaseFemale")],
        },
        Wearable {
            id: String::from("rac_upper_body"),
            name: String::from("RAC Outfit"),
            description: String::from(
                "RAC's signature pink outfit with matching trousers and long coat. ",
            ),
            category: String::from("upper_body"),
            rarity: String::from("unique"),
            body_shapes: vec![String::from("BaseMale"), String::from("BaseFemale")],
        },
        Wearable {
            id: String::from("kraken_epic_hat"),
            name: String::from("Kraken Ruby-Eye Crown"),
            description: String::from("The Kraken is released upon Decentraland"),
            category: String::from("hat"),
            rarity: String::from("epic"),
            body_shapes: vec![String::from("BaseMale"), String::from("BaseFemale")],
        },
        Wearable {
            id: String::from("kraken_hat"),
            name: String::from("Kraken Hat"),
            description: String::from("The Kraken is released upon Decentraland"),
            category: String::from("hat"),
            rarity: String::from("uncommon"),
            body_shapes: vec![String::from("BaseMale"), String::from("BaseFemale")],
        },
        Wearable {
            id: String::from("kraken_legendary_hat"),
            name: String::from("Kraken Gold-Eye Crown"),
            description: String::from("The Kraken is released upon Decentraland"),
            category: String::from("hat"),
            rarity: String::from("legendary"),
            body_shapes: vec![String::from("BaseMale"), String::from("BaseFemale")],
        },
        Wearable {
            id: String::from("kraken_mythic_hat"),
            name: String::from("Kraken Cyborg Crown"),
            description: String::from("The Kraken is released upon Decentraland"),
            category: String::from("hat"),
            rarity: String::from("mythic"),
            body_shapes: vec![String::from("BaseMale"), String::from("BaseFemale")],
        },
        Wearable {
            id: String::from("kraken_hoodie_upper_body"),
            name: String::from("Kraken Hoodie"),
            description: String::from("The Kraken is released upon Decentraland"),
            category: String::from("upper_body"),
            rarity: String::from("uncommon"),
            body_shapes: vec![String::from("BaseMale"), String::from("BaseFemale")],
        },
        Wearable {
            id: String::from("kraken_tshirt_upper_body"),
            name: String::from("Kraken T-Shirt"),
            description: String::from("The Kraken is released upon Decentraland"),
            category: String::from("upper_body"),
            rarity: String::from("uncommon"),
            body_shapes: vec![String::from("BaseMale"), String::from("BaseFemale")],
        },
    ]
}
