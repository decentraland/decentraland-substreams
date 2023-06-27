use super::wearable::Wearable;

pub fn winklevoss_capital() -> Vec<Wearable> {
    vec![
        Wearable {
            id: String::from("winklevoss_hat"),
            name: String::from("Winklevoss Cap"),
            description: String::from("Winklevoss Capital branded cap in signature pink."),
            category: String::from("hat"),
            rarity: String::from("epic"),
            body_shapes: vec![String::from("BaseMale"), String::from("BaseFemale")],
        },
        Wearable {
            id: String::from("winklevoss_upper_body"),
            name: String::from("Winklevoss T-Shirt"),
            description: String::from("Winklevoss Capital branded T-shirt in signature pink."),
            category: String::from("upper_body"),
            rarity: String::from("epic"),
            body_shapes: vec![String::from("BaseMale"), String::from("BaseFemale")],
        },
    ]
}
