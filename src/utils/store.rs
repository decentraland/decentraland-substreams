pub fn get_store_address(network: &str) -> String {
    if network == "matic" {
        return String::from("0x214ffc0f0103735728dc66b61a22e4f163e275ae");
    }

    if network == "mumbai" {
        return String::from("0x6ddf1b1924dad850adbc1c02026535464be06b0c");
    }

    substreams::log::info!(format!(
        "Could not find store address. Invalid network {}",
        network
    ));
    String::new()
}
