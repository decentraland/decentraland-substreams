use super::wearable::Wearable;

pub fn dg_fall_2020() -> Vec<Wearable> {
    vec![
        Wearable::new(
            String::from("dg_money_shades_eyewear"),
            String::from("Money Shades glasses"),
            String::from("Shades worn while making money #moneyshades"),
            String::from("eyewear"),
            String::from("mythic"),
            vec![String::from("BaseMale"), String::from("BaseFemale")],
        ),
        Wearable::new(
            String::from("dg_dress_shoes_feet"),
            String::from("Dress Shoes"),
            String::from("Dress shoes essential with the DG suit #dgsuit"),
            String::from("feet"),
            String::from("mythic"),
            vec![String::from("BaseMale"), String::from("BaseFemale")],
        ),
        Wearable::new(
            String::from("dg_kicks_feet"),
            String::from("Shoes"),
            String::from("Comfortable mid top kicks with decadent DG cashmere socks #dgkicks"),
            String::from("feet"),
            String::from("mythic"),
            vec![String::from("BaseMale"), String::from("BaseFemale")],
        ),
        Wearable::new(
            String::from("dg_suit_bottom_lower_body"),
            String::from("Suit bottom"),
            String::from("A decadent & elegant pair of DG suit pants #dgsuit"),
            String::from("lower_body"),
            String::from("mythic"),
            vec![String::from("BaseMale"), String::from("BaseFemale")],
        ),
        Wearable::new(
            String::from("dg_robe_upper_body"),
            String::from("Robe"),
            String::from(
                "A silky smooth robe that lets everyone know who's the real baus #cozybaus ",
            ),
            String::from("upper_body"),
            String::from("mythic"),
            vec![String::from("BaseMale"), String::from("BaseFemale")],
        ),
        Wearable::new(
            String::from("dg_suit_top_upper_body"),
            String::from("Suit Top"),
            String::from("A decadent & elegant DG suit jacket with golden accents #dgsuit"),
            String::from("upper_body"),
            String::from("mythic"),
            vec![String::from("BaseMale"), String::from("BaseFemale")],
        ),
    ]
}
