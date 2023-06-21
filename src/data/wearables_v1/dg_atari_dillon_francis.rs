use super::wearable::Wearable;

pub const dg_atari_dillon_francis: Vec<Wearable> = vec![
    Wearable::new(
        String::from("gerald_patchwork_knit_sweater"),
        String::from("Gerald Patchwork Knit Sweater"),
        String::from("A cozy Barney Cools Gerald Patchwork Knit Sweater."),
        String::from("upper_body"),
        String::from("epic"),
        vec![String::from("BaseMale"), String::from("BaseFemale")],
    ),
    Wearable::new(
        String::from("gerald_striped_knit_sweater"),
        String::from("Gerald Striped Knit Sweater"),
        String::from("A cozy Barney Cools Gerald Striped Knit Sweater."),
        String::from("upper_body"),
        String::from("epic"),
        vec![String::from("BaseMale"), String::from("BaseFemale")],
    ),
];
