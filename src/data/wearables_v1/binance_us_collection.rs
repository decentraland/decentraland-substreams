use super::wearable::Wearable;

pub fn binance_us_collection() -> Vec<Wearable> {
    vec![
        Wearable::new(
            String::from("binance_us_hat"),
            String::from("Binance US Hat"),
            String::from("Binance US Hat"),
            String::from("hat"),
            String::from("uncommon"),
            vec![String::from("BaseMale"), String::from("BaseFemale")],
        ),
        Wearable::new(
            String::from("binance_us_upper_body"),
            String::from("Binance US Hoodie"),
            String::from("Binance US Hoodie"),
            String::from("upper_body"),
            String::from("uncommon"),
            vec![String::from("BaseMale"), String::from("BaseFemale")],
        ),
    ]
}
