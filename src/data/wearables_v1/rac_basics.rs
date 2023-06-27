use super::wearable::Wearable;

pub fn rac_basics() -> Vec<Wearable> {
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
    ]
}
