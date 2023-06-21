use super::wearable::Wearable;

pub const dc_niftyblocksmith: Vec<Wearable> = vec![
    Wearable::new(
        String::from("blocksmith_eyewear "),
        String::from("DC Nifty Blocksmith Diamantex "),
        String::from("Diamantex TM "),
        String::from("eyewear "),
        String::from("legendary "),
        vec![String::from("BaseMale"), String::from("BaseFemale")],
    ),
    Wearable::new(
        String::from("blocksmith_feet "),
        String::from("DC Nifty Blocksmith Boots "),
        String::from("These boots will leave digital footprints wherever you go "),
        String::from("feet "),
        String::from("legendary "),
        vec![String::from("BaseMale"), String::from("BaseFemale")],
    ),
    Wearable::new(
        String::from("blocksmith_helmet "),
        String::from("DC Nifty Blocksmith Helmet "),
        String::from("This unique helmet will keep your thoughts positively focused. Zero toxicity. "),
        String::from("helmet "),
        String::from("legendary "),
        vec![String::from("BaseMale"), String::from("BaseFemale")],
    ),
    Wearable::new(
        String::from("blocksmith_lower_body "),
        String::from("DC Nifty Blocksmith Trousers "),
        String::from("Need some repair. "),
        String::from("lower_body "),
        String::from("legendary "),
        vec![String::from("BaseMale"), String::from("BaseFemale")],
    ),
    Wearable::new(
        String::from("blocksmith_top_head "),
        String::from("DC Nifty Blocksmith Drone "),
        String::from("BlockSmith’s pet. "),
        String::from("top_head "),
        String::from("legendary "),
        vec![String::from("BaseMale"), String::from("BaseFemale")],
    ),
    Wearable::new(
        String::from("blocksmith_upper_body "),
        String::from("DC Nifty Blocksmith Jacket "),
        String::from("Tailor made one sleeve style. "),
        String::from("upper_body "),
        String::from("legendary "),
        vec![String::from("BaseMale"), String::from("BaseFemale")],
    ),
];
