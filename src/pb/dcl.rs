// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigInt {
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Items {
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<Item>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Item {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub collection: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub blockchain_id: i64,
    #[prost(string, tag = "4")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub item_type: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub total_supply: ::core::option::Option<BigInt>,
    #[prost(message, optional, tag = "7")]
    pub max_supply: ::core::option::Option<BigInt>,
    #[prost(string, tag = "8")]
    pub rarity: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "9")]
    pub creation_fee: ::core::option::Option<BigInt>,
    #[prost(message, optional, tag = "10")]
    pub available: ::core::option::Option<BigInt>,
    #[prost(message, optional, tag = "11")]
    pub price: ::core::option::Option<BigInt>,
    #[prost(string, tag = "12")]
    pub beneficiary: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "13")]
    pub content_hash: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag = "14")]
    pub uri: ::prost::alloc::string::String,
    #[prost(string, tag = "15")]
    pub image: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "16")]
    pub minters: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "17")]
    pub managers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "18")]
    pub metadata: ::core::option::Option<Metadata>,
    #[prost(string, tag = "19")]
    pub raw_metadata: ::prost::alloc::string::String,
    #[prost(string, tag = "20")]
    pub urn: ::prost::alloc::string::String,
    #[prost(uint64, tag = "21")]
    pub created_at: u64,
    #[prost(uint64, tag = "22")]
    pub updated_at: u64,
    #[prost(uint64, tag = "23")]
    pub reviewed_at: u64,
    #[prost(uint64, optional, tag = "24")]
    pub sold_at: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "25")]
    pub first_listed_at: ::core::option::Option<u64>,
    #[prost(uint64, tag = "26")]
    pub block_number: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Collections {
    #[prost(message, repeated, tag = "1")]
    pub collections: ::prost::alloc::vec::Vec<Collection>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Collection {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub owner: ::prost::alloc::string::String,
    #[prost(bool, tag = "6")]
    pub is_completed: bool,
    #[prost(bool, tag = "7")]
    pub is_editable: bool,
    #[prost(string, repeated, tag = "8")]
    pub minters: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "9")]
    pub managers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "10")]
    pub urn: ::prost::alloc::string::String,
    #[prost(uint64, tag = "11")]
    pub created_at: u64,
    #[prost(uint64, tag = "12")]
    pub updated_at: u64,
    #[prost(uint64, tag = "13")]
    pub reviewed_at: u64,
    #[prost(uint64, optional, tag = "14")]
    pub first_listed_at: ::core::option::Option<u64>,
    #[prost(uint64, tag = "15")]
    pub block_number: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectionTransferOwnershipEvent {
    #[prost(string, tag = "1")]
    pub collection: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub to: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectionTransferOwnershipEvents {
    #[prost(message, repeated, tag = "1")]
    pub events: ::prost::alloc::vec::Vec<CollectionTransferOwnershipEvent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectionTransferCreatorshipEvent {
    #[prost(string, tag = "1")]
    pub collection: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub to: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectionTransferCreatorshipEvents {
    #[prost(message, repeated, tag = "1")]
    pub events: ::prost::alloc::vec::Vec<CollectionTransferCreatorshipEvent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectionSetApprovedEvent {
    #[prost(string, tag = "1")]
    pub collection: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub new_value: bool,
    #[prost(string, tag = "3")]
    pub updated_at: ::prost::alloc::string::String,
    #[prost(uint64, tag = "4")]
    pub block_number: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectionSetApprovedEvents {
    #[prost(message, repeated, tag = "1")]
    pub events: ::prost::alloc::vec::Vec<CollectionSetApprovedEvent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectionSetGlobalMinterEvent {
    #[prost(string, tag = "1")]
    pub collection: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub minter: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub value: bool,
    #[prost(string, tag = "4")]
    pub timestamp: ::prost::alloc::string::String,
    #[prost(uint64, tag = "5")]
    pub block_number: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectionSetGlobalMinterEvents {
    #[prost(message, repeated, tag = "1")]
    pub events: ::prost::alloc::vec::Vec<CollectionSetGlobalMinterEvent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ItemUpdateDataEvent {
    #[prost(string, tag = "1")]
    pub collection: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub item: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub price: ::core::option::Option<BigInt>,
    #[prost(string, tag = "4")]
    pub beneficiary: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub raw_metadata: ::prost::alloc::string::String,
    #[prost(uint64, tag = "6")]
    pub timestamp: u64,
    #[prost(uint64, tag = "7")]
    pub block_number: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ItemUpdateDataEvents {
    #[prost(message, repeated, tag = "1")]
    pub events: ::prost::alloc::vec::Vec<ItemUpdateDataEvent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RescueItemEvent {
    #[prost(string, tag = "1")]
    pub collection: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub item: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub raw_metadata: ::prost::alloc::string::String,
    #[prost(uint64, tag = "4")]
    pub timestamp: u64,
    #[prost(uint64, tag = "5")]
    pub block_number: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RescueItemEvents {
    #[prost(message, repeated, tag = "1")]
    pub events: ::prost::alloc::vec::Vec<RescueItemEvent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferEvent {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub token_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub contract: ::prost::alloc::string::String,
    #[prost(uint64, tag = "5")]
    pub timestamp: u64,
    #[prost(uint64, tag = "6")]
    pub block_number: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferEvents {
    #[prost(message, repeated, tag = "1")]
    pub events: ::prost::alloc::vec::Vec<TransferEvent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetItemMinterEvent {
    #[prost(string, tag = "1")]
    pub item: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub collection: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub minter: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub value: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub timestamp: ::prost::alloc::string::String,
    #[prost(uint64, tag = "6")]
    pub block_number: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetItemMinterEvents {
    #[prost(message, repeated, tag = "1")]
    pub events: ::prost::alloc::vec::Vec<SetItemMinterEvent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ItemMinters {
    #[prost(string, tag = "1")]
    pub item: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub minters: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NfTs {
    #[prost(message, repeated, tag = "1")]
    pub nfts: ::prost::alloc::vec::Vec<Nft>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Nft {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub beneficiary: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub token_id: ::core::option::Option<BigInt>,
    #[prost(string, tag = "4")]
    pub item_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub issued_id: ::core::option::Option<BigInt>,
    #[prost(string, tag = "6")]
    pub collection_address: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub created_at: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub updated_at: ::prost::alloc::string::String,
    #[prost(uint64, tag = "9")]
    pub block_number: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Orders {
    #[prost(message, repeated, tag = "1")]
    pub orders: ::prost::alloc::vec::Vec<Order>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Order {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub marketplace_address: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub nft: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub token_id: ::core::option::Option<BigInt>,
    #[prost(string, tag = "5")]
    pub tx_hash: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub buyer: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "8")]
    pub price: ::core::option::Option<BigInt>,
    #[prost(string, tag = "9")]
    pub status: ::prost::alloc::string::String,
    #[prost(uint64, tag = "10")]
    pub block_number: u64,
    #[prost(uint64, tag = "11")]
    pub created_at: u64,
    #[prost(message, optional, tag = "12")]
    pub expires_at: ::core::option::Option<BigInt>,
    #[prost(uint64, tag = "13")]
    pub updated_at: u64,
    #[prost(string, tag = "14")]
    pub item: ::prost::alloc::string::String,
    #[prost(string, tag = "15")]
    pub nft_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metadata {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub item_type: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub wearable: ::core::option::Option<Wearable>,
    #[prost(message, optional, tag = "4")]
    pub emote: ::core::option::Option<Emote>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Wearable {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub collection: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub category: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "6")]
    pub body_shapes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Emote {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub collection: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub category: ::prost::alloc::string::String,
    #[prost(bool, tag = "6")]
    pub r#loop: bool,
    #[prost(string, repeated, tag = "7")]
    pub body_shapes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, tag = "8")]
    pub has_sound: bool,
    #[prost(bool, tag = "9")]
    pub has_geometry: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ItemType {
    Undefined = 0,
    WearableV1 = 1,
    WearableV2 = 2,
    SmartWearableV1 = 3,
    EmoteV1 = 4,
}
impl ItemType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ItemType::Undefined => "UNDEFINED",
            ItemType::WearableV1 => "WEARABLE_V1",
            ItemType::WearableV2 => "WEARABLE_V2",
            ItemType::SmartWearableV1 => "SMART_WEARABLE_V1",
            ItemType::EmoteV1 => "EMOTE_V1",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNDEFINED" => Some(Self::Undefined),
            "WEARABLE_V1" => Some(Self::WearableV1),
            "WEARABLE_V2" => Some(Self::WearableV2),
            "SMART_WEARABLE_V1" => Some(Self::SmartWearableV1),
            "EMOTE_V1" => Some(Self::EmoteV1),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum WearableCategory {
    Eyebrows = 0,
    Eyes = 1,
    FacialHair = 2,
    Hair = 3,
    Mouth = 4,
    UpperBody = 5,
    LowerBody = 6,
    Feet = 7,
    Earring = 8,
    Eyewear = 9,
    Hat = 10,
    Helmet = 11,
    Mask = 12,
    Tiara = 13,
    TopHead = 14,
    Skin = 15,
}
impl WearableCategory {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            WearableCategory::Eyebrows => "EYEBROWS",
            WearableCategory::Eyes => "EYES",
            WearableCategory::FacialHair => "FACIAL_HAIR",
            WearableCategory::Hair => "HAIR",
            WearableCategory::Mouth => "MOUTH",
            WearableCategory::UpperBody => "UPPER_BODY",
            WearableCategory::LowerBody => "LOWER_BODY",
            WearableCategory::Feet => "FEET",
            WearableCategory::Earring => "EARRING",
            WearableCategory::Eyewear => "EYEWEAR",
            WearableCategory::Hat => "HAT",
            WearableCategory::Helmet => "HELMET",
            WearableCategory::Mask => "MASK",
            WearableCategory::Tiara => "TIARA",
            WearableCategory::TopHead => "TOP_HEAD",
            WearableCategory::Skin => "SKIN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EYEBROWS" => Some(Self::Eyebrows),
            "EYES" => Some(Self::Eyes),
            "FACIAL_HAIR" => Some(Self::FacialHair),
            "HAIR" => Some(Self::Hair),
            "MOUTH" => Some(Self::Mouth),
            "UPPER_BODY" => Some(Self::UpperBody),
            "LOWER_BODY" => Some(Self::LowerBody),
            "FEET" => Some(Self::Feet),
            "EARRING" => Some(Self::Earring),
            "EYEWEAR" => Some(Self::Eyewear),
            "HAT" => Some(Self::Hat),
            "HELMET" => Some(Self::Helmet),
            "MASK" => Some(Self::Mask),
            "TIARA" => Some(Self::Tiara),
            "TOP_HEAD" => Some(Self::TopHead),
            "SKIN" => Some(Self::Skin),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EmoteCategory {
    Dance = 0,
    Stunt = 1,
    Greetings = 2,
    Fun = 3,
    Poses = 4,
    Reactions = 5,
    Horror = 6,
    Miscellaneous = 7,
}
impl EmoteCategory {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EmoteCategory::Dance => "DANCE",
            EmoteCategory::Stunt => "STUNT",
            EmoteCategory::Greetings => "GREETINGS",
            EmoteCategory::Fun => "FUN",
            EmoteCategory::Poses => "POSES",
            EmoteCategory::Reactions => "REACTIONS",
            EmoteCategory::Horror => "HORROR",
            EmoteCategory::Miscellaneous => "MISCELLANEOUS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DANCE" => Some(Self::Dance),
            "STUNT" => Some(Self::Stunt),
            "GREETINGS" => Some(Self::Greetings),
            "FUN" => Some(Self::Fun),
            "POSES" => Some(Self::Poses),
            "REACTIONS" => Some(Self::Reactions),
            "HORROR" => Some(Self::Horror),
            "MISCELLANEOUS" => Some(Self::Miscellaneous),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum WearableRarity {
    Common = 0,
    Uncommon = 1,
    Rare = 2,
    Epic = 3,
    Legendary = 4,
    Mythic = 5,
    Unique = 6,
    Exotic = 7,
}
impl WearableRarity {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            WearableRarity::Common => "COMMON",
            WearableRarity::Uncommon => "UNCOMMON",
            WearableRarity::Rare => "RARE",
            WearableRarity::Epic => "EPIC",
            WearableRarity::Legendary => "LEGENDARY",
            WearableRarity::Mythic => "MYTHIC",
            WearableRarity::Unique => "UNIQUE",
            WearableRarity::Exotic => "EXOTIC",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "COMMON" => Some(Self::Common),
            "UNCOMMON" => Some(Self::Uncommon),
            "RARE" => Some(Self::Rare),
            "EPIC" => Some(Self::Epic),
            "LEGENDARY" => Some(Self::Legendary),
            "MYTHIC" => Some(Self::Mythic),
            "UNIQUE" => Some(Self::Unique),
            "EXOTIC" => Some(Self::Exotic),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum WearableBodyShape {
    BaseFemale = 0,
    BaseMale = 1,
}
impl WearableBodyShape {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            WearableBodyShape::BaseFemale => "BASE_FEMALE",
            WearableBodyShape::BaseMale => "BASE_MALE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BASE_FEMALE" => Some(Self::BaseFemale),
            "BASE_MALE" => Some(Self::BaseMale),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
