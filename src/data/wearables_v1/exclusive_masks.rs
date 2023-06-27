use super::wearable::Wearable;

pub fn exclusive_masks() -> Vec<Wearable> {
    vec![
        Wearable::new(
            String::from("bird_mask"),
            String::from("Bird Mask"),
            String::from(""),
            String::from("mask"),
            String::from("legendary"),
            vec![String::from("BaseMale"), String::from("BaseFemale")],
        ),
        Wearable::new(
            String::from("classic_mask"),
            String::from("Classic Mask"),
            String::from(""),
            String::from("mask"),
            String::from("legendary"),
            vec![String::from("BaseMale"), String::from("BaseFemale")],
        ),
        Wearable::new(
            String::from("clown_nose"),
            String::from("Clown Nose"),
            String::from(""),
            String::from("mask"),
            String::from("legendary"),
            vec![String::from("BaseMale"), String::from("BaseFemale")],
        ),
        Wearable::new(
            String::from("asian_fox"),
            String::from("Asian Fox Mask"),
            String::from(""),
            String::from("mask"),
            String::from("legendary"),
            vec![String::from("BaseMale"), String::from("BaseFemale")],
        ),
        Wearable::new(
            String::from("killer_mask"),
            String::from("Killer Mask"),
            String::from(""),
            String::from("mask"),
            String::from("legendary"),
            vec![String::from("BaseMale"), String::from("BaseFemale")],
        ),
        Wearable::new(
            String::from("serial_killer_mask"),
            String::from("Serial Killer Mask"),
            String::from(""),
            String::from("mask"),
            String::from("legendary"),
            vec![String::from("BaseMale"), String::from("BaseFemale")],
        ),
        Wearable::new(
            String::from("theater_mask"),
            String::from("Theater"),
            String::from(""),
            String::from("mask"),
            String::from("legendary"),
            vec![String::from("BaseMale"), String::from("BaseFemale")],
        ),
        Wearable::new(
            String::from("tropical_mask"),
            String::from("Tropical Mask"),
            String::from(""),
            String::from("mask"),
            String::from("legendary"),
            vec![String::from("BaseMale"), String::from("BaseFemale")],
        ),
    ]
}
