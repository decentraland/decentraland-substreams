use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    // this one is commented out because it needs to be crafted manually due to the Tuple usage
    // Abigen::new("CollectionV2", "abi/collectionV2.json")?
    //     .generate()?
    //     .write_to_file("src/abi/collectionV2.rs")?;

    // Marketplace contract
    Abigen::new("Marketplace", "abi/Marketplace.json")?
        .generate()?
        .write_to_file("src/abi/marketplace.rs")?;

    // MarketplaceV2 contract
    Abigen::new("MarketplaceV2", "abi/MarketplaceV2.json")?
        .generate()?
        .write_to_file("src/abi/marketplacev2.rs")?;

    // CollectionFactoryV3 contract
    Abigen::new("CollectionFactoryV3", "abi/CollectionFactoryV3.json")?
        .generate()?
        .write_to_file("src/abi/collectionFactoryv3.rs")?;

    // CollectionFactory contract
    Abigen::new("CollectionFactory", "abi/CollectionFactory.json")?
        .generate()?
        .write_to_file("src/abi/collectionFactory.rs")?;

    // ERC721 collection v1 contract
    Abigen::new("ERC721", "abi/ERC721.json")?
        .generate()?
        .write_to_file("src/abi/ERC721.rs")?;

    // Lands
    Abigen::new("Lands", "abi/Lands.json")?
        .generate()?
        .write_to_file("src/abi/Lands.rs")?;

    // Names
    Abigen::new("Names", "abi/Names.json")?
        .generate()?
        .write_to_file("src/abi/Names.rs")?;

    Ok(())
}
