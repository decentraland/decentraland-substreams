pub fn get_urn_for_collection_v2(collection_address: String, network: String) -> String {
    format!(
        "urn:decentraland:ethereum:{}:collections-v2:{}",
        network, collection_address
    )
}
