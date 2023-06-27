use super::wearable::Wearable;

pub fn meme_dontbuythis() -> Vec<Wearable> {
    vec![
        Wearable {
            id: String::from("meme_helmet_pineapple"),
            name: String::from("Tasty Pineapple Head"),
            description: String::from(
                "Tasty Pineapple Head from the $Meme Collection - Don't wear this ",
            ),
            category: String::from("helmet"),
            rarity: String::from("epic"),
            body_shapes: vec![String::from("BaseMale"), String::from("BaseFemale")],
        },
        Wearable {
            id: String::from("meme_pants_pineapple"),
            name: String::from("Fresh Pineapple Pants"),
            description: String::from(
                "Fresh Pineapple Pants from the $Meme collection - Don't wear this ",
            ),
            category: String::from("lower_body"),
            rarity: String::from("epic"),
            body_shapes: vec![String::from("BaseMale"), String::from("BaseFemale")],
        },
        Wearable {
            id: String::from("meme_suit_pineapple_onesie"),
            name: String::from("Sweet Pineapple Onesie"),
            description: String::from(
                "Sweet Pineapple Onesie from the $Meme collection - Don't wear this ",
            ),
            category: String::from("upper_body"),
            rarity: String::from("epic"),
            body_shapes: vec![String::from("BaseMale"), String::from("BaseFemale")],
        },
        Wearable {
            id: String::from("meme_suit_pineapple"),
            name: String::from("Cool Pineapple Suit"),
            description: String::from(
                "Cool Pineapple Suit from the $Meme collection - Don't wear this ",
            ),
            category: String::from("upper_body"),
            rarity: String::from("epic"),
            body_shapes: vec![String::from("BaseMale"), String::from("BaseFemale")],
        },
    ]
}
