use super::wearable::Wearable;

pub const threelau_basics: Vec<Wearable> = vec![
    Wearable {
        id: String::from("3lau_blue_hat "),
        name: String::from("3LAU Blue Triangle Cap "),
        description: String::from("3LAU's signature triangle on a black and blue cap. "),
        category: String::from("hat "),
        rarity: String::from("legendary "),
        body_shapes: vec![String::from("BaseMale"), String::from("BaseFemale")],
    },
    Wearable {
        id: String::from("3lau_white_hat "),
        name: String::from("3LAU White Triangle Cap "),
        description: String::from("3LAU's signature triangle on a black and white cap. "),
        category: String::from("hat "),
        rarity: String::from("epic "),
        body_shapes: vec![String::from("BaseMale"), String::from("BaseFemale")],
    },
    Wearable {
        id: String::from("3lau_hoodie_b_upper_body "),
        name: String::from("3LAU Blue Triangle Hoodie "),
        description: String::from("3LAU's signature triangle on a black and blue hoodie. "),
        category: String::from("upper_body "),
        rarity: String::from("legendary "),
        body_shapes: vec![String::from("BaseMale"), String::from("BaseFemale")],
    },
    Wearable {
        id: String::from("3lau_hoodie_w_upper_body "),
        name: String::from("3LAU White Triangle Hoodie "),
        description: String::from("3LAU's signature triangle on a black and white hoodie. "),
        category: String::from("upper_body "),
        rarity: String::from("epic "),
        body_shapes: vec![String::from("BaseMale"), String::from("BaseFemale")],
    },
    Wearable {
        id: String::from("3lau_tshirt_b_upper_body "),
        name: String::from("3LAU Blue Triangle Tee "),
        description: String::from("3LAU's signature triangle on a black and blue tee. "),
        category: String::from("upper_body "),
        rarity: String::from("legendary "),
        body_shapes: vec![String::from("BaseMale"), String::from("BaseFemale")],
    },
    Wearable {
        id: String::from("3lau_tshirt_w_upper_body "),
        name: String::from("3LAU White Triangle Tee "),
        description: String::from("3LAU's signature triangle on a black and white tee. "),
        category: String::from("upper_body "),
        rarity: String::from("epic "),
        body_shapes: vec![String::from("BaseMale"), String::from("BaseFemale")],
    },
];
