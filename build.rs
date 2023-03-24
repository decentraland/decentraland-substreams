use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    // Abigen::new("CollectionV2", "abi/collectionV2.json")?
    //     .generate()?
    //     .write_to_file("src/abi/collectionV2.rs")?;

    Abigen::new("CollectionFactoryV3", "abi/CollectionFactoryV3.json")?
        .generate()?
        .write_to_file("src/abi/collectionFactoryv3.rs")?;

    Ok(())
}
