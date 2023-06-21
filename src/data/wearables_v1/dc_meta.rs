use super::wearable::Wearable;

pub const dc_meta: Vec<Wearable> = vec![
    Wearable::new(
        String::from("meta_feet "),
        String::from("DC META 01000001 "),
        String::from("01000100 01000011 00100000 01001101 01000101 01010100 01000001 00100000 01100110 01100101 01100101 01110100 "),
        String::from("feet "),
        String::from("mythic "),
        vec![String::from("BaseMale"), String::from("BaseFemale")],
    ),
    Wearable::new(
        String::from("meta_hair "),
        String::from("DC META 01000100 "),
        String::from("01000100 01000011 00100000 01001101 01000101 01010100 01000001 00100000 01001000 01100001 01101001 01110010 "),
        String::from("hair "),
        String::from("mythic "),
        vec![String::from("BaseMale"), String::from("BaseFemale")],
    ),
    Wearable::new(
        String::from("meta_helmet "),
        String::from("DC META 01000011 "),
        String::from("01000100 01000011 00100000 01001101 01000101 01010100 01000001 00100000 01101101 01101001 01101110 01100100 "),
        String::from("helmet "),
        String::from("mythic "),
        vec![String::from("BaseMale"), String::from("BaseFemale")],
    ),
    Wearable::new(
        String::from("meta_lower_body "),
        String::from("DC META 01010100 "),
        String::from("01000100 01000011 00100000 01001101 01000101 01010100 01000001 00100000 01101100 01100101 01100111 01110011 "),
        String::from("lower_body "),
        String::from("mythic "),
        vec![String::from("BaseMale"), String::from("BaseFemale")],
    ),
    Wearable::new(
        String::from("meta_tiara "),
        String::from("DC META 01001101 "),
        String::from("01000100 01000011 00100000 01001101 01000101 01010100 01000001 00100000 01110011 01101111 01110101 01101100 "),
        String::from("tiara "),
        String::from("mythic "),
        vec![String::from("BaseMale"), String::from("BaseFemale")],
    ),
    Wearable::new(
        String::from("meta_upper_body "),
        String::from("DC META 01000101 "),
        String::from("01000100 01000011 00100000 01001101 01000101 01010100 01000001 00100000 01100010 01101111 01100100 01111001 "),
        String::from("upper_body "),
        String::from("mythic "),
        vec![String::from("BaseMale"), String::from("BaseFemale")],
    ),
];
