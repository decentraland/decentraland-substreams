use super::wearable::Wearable;

pub fn cybermike_cybersoldier_set() -> Vec<Wearable> {
    vec![
        Wearable::new(
            String::from("cybersoldier_nightvision_eyewear"),
            String::from(
                "CYBERSOLDIER NIGHT VISION GOGGLES - Winter Soldier Collection - FROM CYBERMIKE ",
            ),
            String::from("By Cybermike - Live Free or Die Hard"),
            String::from("eyewear"),
            String::from("mythic"),
            vec![String::from("BaseMale"), String::from("BaseFemale")],
        ),
        Wearable::new(
            String::from("cybersoldier_boots_feet"),
            String::from(
                "CYBERSOLDIER HASTE BOOTS - Winter Soldier Collection - Mark III - FROM CYBERMIKE ",
            ),
            String::from("By Cybermike - Live Free or Die Hard"),
            String::from("feet"),
            String::from("mythic"),
            vec![String::from("BaseMale"), String::from("BaseFemale")],
        ),
        Wearable::new(
            String::from("cybersoldier_helmet"),
            String::from(
                "CYBERSOLDIER HELMET - Winter Soldier Collection - Mark III - FROM CYBERMIKE",
            ),
            String::from("By Cybermike - Live Free or Die Hard"),
            String::from("helmet"),
            String::from("mythic"),
            vec![String::from("BaseMale"), String::from("BaseFemale")],
        ),
        Wearable::new(
            String::from("cybersoldier_leggings_lower_body"),
            String::from(
                "CYBERSOLDIER LEGGINGS - Winter Soldier Collection - Mark III - FROM CYBERMIKE ",
            ),
            String::from("By Cybermike - Live Free or Die Hard"),
            String::from("lower_body"),
            String::from("mythic"),
            vec![String::from("BaseMale"), String::from("BaseFemale")],
        ),
        Wearable::new(
            String::from("cybersoldier_gas_mask"),
            String::from("CYBERSOLDIER GAS MASK - Winter Soldier Collection - FROM CYBERMIKE"),
            String::from("By Cybermike - Live Free or Die Hard"),
            String::from("mask"),
            String::from("mythic"),
            vec![String::from("BaseMale"), String::from("BaseFemale")],
        ),
        Wearable::new(
            String::from("cybersoldier_torso_upper_body"),
            String::from(
                "CYBERSOLDIER TORSO - Winter Soldier Collection - Mark III - FROM CYBERMIKE",
            ),
            String::from("By Cybermike - Live Free or Die Hard"),
            String::from("upper_body"),
            String::from("mythic"),
            vec![String::from("BaseMale"), String::from("BaseFemale")],
        ),
    ]
}
