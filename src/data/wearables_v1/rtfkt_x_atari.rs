use super::wearable::Wearable;

pub fn rtfkt_x_atari() -> Vec<Wearable> {
    vec![
        Wearable {
            id: String::from("p_rtfkt_x_atari_feet"),
            name: String::from("Purple RTFKT X Atari Sneakers"),
            description: String::from("RTFKT x Atari sneakers with holographic print."),
            category: String::from("feet"),
            rarity: String::from("epic"),
            body_shapes: vec![String::from("BaseMale"), String::from("BaseFemale")],
        },
        Wearable {
            id: String::from("r_rtfkt_x_atari_feet"),
            name: String::from("Red RTFKT X Atari Sneakers"),
            description: String::from("Unique RTFKT x Atari sneakers in red."),
            category: String::from("feet"),
            rarity: String::from("unique"),
            body_shapes: vec![String::from("BaseMale"), String::from("BaseFemale")],
        },
    ]
}
