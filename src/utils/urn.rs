const BASE_DECENTRALAND_URN: &str = "urn:decentraland:";

fn format_network(network: &str) -> &str {
    if network == "polygon" {
        "matic"
    } else {
        network
    }
}

pub fn get_urn_for_collection_v2(collection_address: &str, item_id: &str, network: &str) -> String {
    let formatted_network = format_network(network);
    format!(
        "{}{}:collections-v2:{}:{}",
        BASE_DECENTRALAND_URN, formatted_network, collection_address, item_id
    )
}

pub fn get_collection_urn_for_collection_v2(collection_address: &str, network: &str) -> String {
    let formatted_network = format_network(network);
    format!(
        "{}{}:collections-v2:{}",
        BASE_DECENTRALAND_URN, formatted_network, collection_address
    )
}

fn get_urn_for_collection_v1(collection_name: &str, network: &str) -> String {
    let collection_name: Vec<&str> = collection_name.split("dcl://").collect();
    // let base_decentraland_urn = "baseDecentralandURN".to_string(); // Replace with the actual value
    // let network = String::from("ethereum"); //@TODO: Read from env
    let urn_suffix = if collection_name.len() > 1 {
        collection_name[1]
    } else {
        collection_name[0]
    };
    format!(
        "{}{}:collections-v1:{}",
        BASE_DECENTRALAND_URN,
        network,
        urn_suffix //@TODO check this, chain is missing I think
    )
}

pub fn get_urn_for_wearable_v1(
    network: &str,
    collection_name: &str,
    representation_id: &str,
) -> String {
    let collection_urn = get_urn_for_collection_v1(collection_name, network);
    format!("{}:{}", collection_urn, representation_id)
}
