use super::wearable::Wearable;

pub fn pm_dreamverse_eminence() -> Vec<Wearable> {
    vec![
        Wearable {
            id: String::from("pm_dreamverse_eminence_boots"),
            name: String::from("Ethereal Dreamer Boots"),
            description: String::from(
                "Expanding the Polygonal Mind Collection adding now the Dreamverse Eminence set ",
            ),
            category: String::from("feet"),
            rarity: String::from("mythic"),
            body_shapes: vec![String::from("BaseMale"), String::from("BaseFemale")],
        },
        Wearable {
            id: String::from("pm_dreamverse_eminence_cap"),
            name: String::from("Ethereal Sage Hat"),
            description: String::from(
                "Expanding the Polygonal Mind Collection adding now the Dreamverse Eminence set ",
            ),
            category: String::from("hat"),
            rarity: String::from("mythic"),
            body_shapes: vec![String::from("BaseMale"), String::from("BaseFemale")],
        },
        Wearable {
            id: String::from("pm_dreamverse_eminence_hat_visor"),
            name: String::from("Lucid Visionary Crown"),
            description: String::from(
                "Expanding the Polygonal Mind Collection adding now the Dreamverse Eminence set ",
            ),
            category: String::from("hat"),
            rarity: String::from("mythic"),
            body_shapes: vec![String::from("BaseMale"), String::from("BaseFemale")],
        },
        Wearable {
            id: String::from("pm_dreamverse_eminence_pants"),
            name: String::from("Dream Bourgeois Breeches"),
            description: String::from(
                "Expanding the Polygonal Mind Collection adding now the Dreamverse Eminence set ",
            ),
            category: String::from("lower_body"),
            rarity: String::from("mythic"),
            body_shapes: vec![String::from("BaseMale"), String::from("BaseFemale")],
        },
        Wearable {
            id: String::from("pm_dreamverse_eminence_jacket"),
            name: String::from("Dream Oracle Jacket"),
            description: String::from(
                "Expanding the Polygonal Mind Collection adding now the Dreamverse Eminence set ",
            ),
            category: String::from("upper_body"),
            rarity: String::from("mythic"),
            body_shapes: vec![String::from("BaseMale"), String::from("BaseFemale")],
        },
        Wearable {
            id: String::from("pm_dreamverse_eminence_sweater"),
            name: String::from("Lucid Meditation Tunic"),
            description: String::from(
                "Expanding the Polygonal Mind Collection adding now the Dreamverse Eminence set ",
            ),
            category: String::from("upper_body"),
            rarity: String::from("mythic"),
            body_shapes: vec![String::from("BaseMale"), String::from("BaseFemale")],
        },
    ]
}
