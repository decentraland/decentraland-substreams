// This is not a auto-generated file. This has been crafted to work-around the tuple current limitation in ethabi crate.

const INTERNAL_ERR: &'static str = "`ethabi_derive` internal error";

/// Contract's events.
#[allow(dead_code, unused_imports, unused_variables)]
pub mod events {
    use super::INTERNAL_ERR;
    #[derive(Debug, Clone, PartialEq)]
    pub struct AddItem {
        pub item_id: substreams::scalar::BigInt,
        pub item: Erc721BaseCollectionV2Item,
    }
    impl AddItem {
        const TOPIC_ID: [u8; 32] = [
            20u8, 100u8, 219u8, 117u8, 202u8, 236u8, 44u8, 79u8, 27u8, 86u8, 181u8, 36u8, 110u8,
            240u8, 26u8, 34u8, 159u8, 245u8, 237u8, 195u8, 176u8, 252u8, 249u8, 178u8, 149u8, 63u8,
            236u8, 83u8, 179u8, 152u8, 125u8, 143u8,
        ];
        pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            if log.topics.len() != 2usize {
                return false;
            }
            if log.data.len() < 320usize {
                return false;
            }
            // if log.data.len() != 32usize {
            //     return false;
            // }
            return log.topics.get(0).expect("bounds already checked").as_ref() == Self::TOPIC_ID;
        }
        pub fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            let mut values = ethabi::decode(
                &[ethabi::ParamType::Tuple(vec![
                    ethabi::ParamType::String,
                    ethabi::ParamType::Uint(256usize),
                    ethabi::ParamType::Uint(256usize),
                    ethabi::ParamType::Uint(256usize),
                    ethabi::ParamType::Address,
                    ethabi::ParamType::String,
                    ethabi::ParamType::String,
                ])],
                log.data.as_ref(),
            )
            .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
            let tuple = values.pop().unwrap();
            if let ethabi::Token::Tuple(mut tokens) = tuple {
                Ok(Self {
                    item_id: {
                        let mut v = [0 as u8; 32];
                        ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'item_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR)
                        .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    item: {
                        tokens.reverse();
                        Erc721BaseCollectionV2Item::decode(tokens).expect(INTERNAL_ERR)
                    },
                })
            } else {
                Err(String::from("")) // return empty if the token is not a tuple.
            }
        }
    }
    impl substreams_ethereum::Event for AddItem {
        const NAME: &'static str = "AddItem";
        fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            Self::match_log(log)
        }
        fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            Self::decode(log)
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Erc721BaseCollectionV2Item {
        pub rarity: String,
        pub max_supply: substreams::scalar::BigInt,
        pub total_supply: substreams::scalar::BigInt,
        pub price: substreams::scalar::BigInt,
        pub beneficiary: Vec<u8>,
        pub metadata: String,
        pub content_hash: String,
    }
    impl Erc721BaseCollectionV2Item {
        pub fn decode(mut values: Vec<ethabi::Token>) -> Result<Self, String> {
            Ok(Self {
                rarity: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_string()
                    .expect(INTERNAL_ERR),
                max_supply: {
                    let mut v = [0 as u8; 32];
                    values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR)
                        .to_big_endian(v.as_mut_slice());
                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                },
                total_supply: {
                    let mut v = [0 as u8; 32];
                    values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR)
                        .to_big_endian(v.as_mut_slice());
                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                },
                price: {
                    let mut v = [0 as u8; 32];
                    values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR)
                        .to_big_endian(v.as_mut_slice());
                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                },
                beneficiary: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                metadata: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_string()
                    .expect(INTERNAL_ERR),
                content_hash: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_string()
                    .expect(INTERNAL_ERR),
            })
        }
    }
}
