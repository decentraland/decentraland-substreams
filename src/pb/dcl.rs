// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigInt {
    #[prost(string, tag="1")]
    pub value: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Items {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<Item>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Item {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub rarity: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub max_supply: ::core::option::Option<BigInt>,
    #[prost(message, optional, tag="4")]
    pub total_supply: ::core::option::Option<BigInt>,
    #[prost(message, optional, tag="5")]
    pub price: ::core::option::Option<BigInt>,
    #[prost(string, tag="6")]
    pub beneficiary: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub metadata: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub content_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="9")]
    pub blockchain_item_id: ::core::option::Option<BigInt>,
    #[prost(string, tag="10")]
    pub collection_id: ::prost::alloc::string::String,
    #[prost(uint64, tag="11")]
    pub created_at: u64,
    #[prost(string, tag="12")]
    pub item_type: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Collections {
    #[prost(message, repeated, tag="1")]
    pub collections: ::prost::alloc::vec::Vec<Collection>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Collection {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub owner: ::prost::alloc::string::String,
    #[prost(bool, tag="6")]
    pub is_completed: bool,
    #[prost(bool, tag="7")]
    pub is_approved: bool,
    #[prost(uint64, tag="8")]
    pub created_at: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Issues {
    #[prost(message, repeated, tag="1")]
    pub issues: ::prost::alloc::vec::Vec<Issue>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Issue {
    #[prost(string, tag="1")]
    pub beneficiary: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub token_id: ::core::option::Option<BigInt>,
    #[prost(message, optional, tag="3")]
    pub item_id: ::core::option::Option<BigInt>,
    #[prost(message, optional, tag="4")]
    pub issued_id: ::core::option::Option<BigInt>,
}
/// Encoded file descriptor set for the `dcl` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xfd, 0x15, 0x0a, 0x09, 0x64, 0x63, 0x6c, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x03,
    0x64, 0x63, 0x6c, 0x22, 0x1e, 0x0a, 0x06, 0x42, 0x69, 0x67, 0x49, 0x6e, 0x74, 0x12, 0x14, 0x0a,
    0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x76, 0x61,
    0x6c, 0x75, 0x65, 0x22, 0x28, 0x0a, 0x05, 0x49, 0x74, 0x65, 0x6d, 0x73, 0x12, 0x1f, 0x0a, 0x05,
    0x69, 0x74, 0x65, 0x6d, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x09, 0x2e, 0x64, 0x63,
    0x6c, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x52, 0x05, 0x69, 0x74, 0x65, 0x6d, 0x73, 0x22, 0xaa, 0x03,
    0x0a, 0x04, 0x49, 0x74, 0x65, 0x6d, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x02, 0x69, 0x64, 0x12, 0x16, 0x0a, 0x06, 0x72, 0x61, 0x72, 0x69, 0x74, 0x79,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x72, 0x61, 0x72, 0x69, 0x74, 0x79, 0x12, 0x2a,
    0x0a, 0x0a, 0x6d, 0x61, 0x78, 0x5f, 0x73, 0x75, 0x70, 0x70, 0x6c, 0x79, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x64, 0x63, 0x6c, 0x2e, 0x42, 0x69, 0x67, 0x49, 0x6e, 0x74, 0x52,
    0x09, 0x6d, 0x61, 0x78, 0x53, 0x75, 0x70, 0x70, 0x6c, 0x79, 0x12, 0x2e, 0x0a, 0x0c, 0x74, 0x6f,
    0x74, 0x61, 0x6c, 0x5f, 0x73, 0x75, 0x70, 0x70, 0x6c, 0x79, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x0b, 0x2e, 0x64, 0x63, 0x6c, 0x2e, 0x42, 0x69, 0x67, 0x49, 0x6e, 0x74, 0x52, 0x0b, 0x74,
    0x6f, 0x74, 0x61, 0x6c, 0x53, 0x75, 0x70, 0x70, 0x6c, 0x79, 0x12, 0x21, 0x0a, 0x05, 0x70, 0x72,
    0x69, 0x63, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x64, 0x63, 0x6c, 0x2e,
    0x42, 0x69, 0x67, 0x49, 0x6e, 0x74, 0x52, 0x05, 0x70, 0x72, 0x69, 0x63, 0x65, 0x12, 0x20, 0x0a,
    0x0b, 0x62, 0x65, 0x6e, 0x65, 0x66, 0x69, 0x63, 0x69, 0x61, 0x72, 0x79, 0x18, 0x06, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x0b, 0x62, 0x65, 0x6e, 0x65, 0x66, 0x69, 0x63, 0x69, 0x61, 0x72, 0x79, 0x12,
    0x1a, 0x0a, 0x08, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x18, 0x07, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x08, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x12, 0x21, 0x0a, 0x0c, 0x63,
    0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x5f, 0x68, 0x61, 0x73, 0x68, 0x18, 0x08, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x0b, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x48, 0x61, 0x73, 0x68, 0x12, 0x39,
    0x0a, 0x12, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x63, 0x68, 0x61, 0x69, 0x6e, 0x5f, 0x69, 0x74, 0x65,
    0x6d, 0x5f, 0x69, 0x64, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x64, 0x63, 0x6c,
    0x2e, 0x42, 0x69, 0x67, 0x49, 0x6e, 0x74, 0x52, 0x10, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x63, 0x68,
    0x61, 0x69, 0x6e, 0x49, 0x74, 0x65, 0x6d, 0x49, 0x64, 0x12, 0x23, 0x0a, 0x0d, 0x63, 0x6f, 0x6c,
    0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x09,
    0x52, 0x0c, 0x63, 0x6f, 0x6c, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x49, 0x64, 0x12, 0x1d,
    0x0a, 0x0a, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x5f, 0x61, 0x74, 0x18, 0x0b, 0x20, 0x01,
    0x28, 0x04, 0x52, 0x09, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x41, 0x74, 0x12, 0x1b, 0x0a,
    0x09, 0x69, 0x74, 0x65, 0x6d, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x09,
    0x52, 0x08, 0x69, 0x74, 0x65, 0x6d, 0x54, 0x79, 0x70, 0x65, 0x22, 0x40, 0x0a, 0x0b, 0x43, 0x6f,
    0x6c, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x12, 0x31, 0x0a, 0x0b, 0x63, 0x6f, 0x6c,
    0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0f,
    0x2e, 0x64, 0x63, 0x6c, 0x2e, 0x43, 0x6f, 0x6c, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x52,
    0x0b, 0x63, 0x6f, 0x6c, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x22, 0xe5, 0x01, 0x0a,
    0x0a, 0x43, 0x6f, 0x6c, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x18, 0x0a, 0x07, 0x61,
    0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x61, 0x64,
    0x64, 0x72, 0x65, 0x73, 0x73, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x16, 0x0a, 0x06, 0x73, 0x79, 0x6d,
    0x62, 0x6f, 0x6c, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x73, 0x79, 0x6d, 0x62, 0x6f,
    0x6c, 0x12, 0x18, 0x0a, 0x07, 0x63, 0x72, 0x65, 0x61, 0x74, 0x6f, 0x72, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x07, 0x63, 0x72, 0x65, 0x61, 0x74, 0x6f, 0x72, 0x12, 0x14, 0x0a, 0x05, 0x6f,
    0x77, 0x6e, 0x65, 0x72, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x6f, 0x77, 0x6e, 0x65,
    0x72, 0x12, 0x21, 0x0a, 0x0c, 0x69, 0x73, 0x5f, 0x63, 0x6f, 0x6d, 0x70, 0x6c, 0x65, 0x74, 0x65,
    0x64, 0x18, 0x06, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0b, 0x69, 0x73, 0x43, 0x6f, 0x6d, 0x70, 0x6c,
    0x65, 0x74, 0x65, 0x64, 0x12, 0x1f, 0x0a, 0x0b, 0x69, 0x73, 0x5f, 0x61, 0x70, 0x70, 0x72, 0x6f,
    0x76, 0x65, 0x64, 0x18, 0x07, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0a, 0x69, 0x73, 0x41, 0x70, 0x70,
    0x72, 0x6f, 0x76, 0x65, 0x64, 0x12, 0x1d, 0x0a, 0x0a, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64,
    0x5f, 0x61, 0x74, 0x18, 0x08, 0x20, 0x01, 0x28, 0x04, 0x52, 0x09, 0x63, 0x72, 0x65, 0x61, 0x74,
    0x65, 0x64, 0x41, 0x74, 0x22, 0x2c, 0x0a, 0x06, 0x49, 0x73, 0x73, 0x75, 0x65, 0x73, 0x12, 0x22,
    0x0a, 0x06, 0x69, 0x73, 0x73, 0x75, 0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0a,
    0x2e, 0x64, 0x63, 0x6c, 0x2e, 0x49, 0x73, 0x73, 0x75, 0x65, 0x52, 0x06, 0x69, 0x73, 0x73, 0x75,
    0x65, 0x73, 0x22, 0x9e, 0x01, 0x0a, 0x05, 0x49, 0x73, 0x73, 0x75, 0x65, 0x12, 0x20, 0x0a, 0x0b,
    0x62, 0x65, 0x6e, 0x65, 0x66, 0x69, 0x63, 0x69, 0x61, 0x72, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x0b, 0x62, 0x65, 0x6e, 0x65, 0x66, 0x69, 0x63, 0x69, 0x61, 0x72, 0x79, 0x12, 0x25,
    0x0a, 0x07, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x49, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x0b, 0x2e, 0x64, 0x63, 0x6c, 0x2e, 0x42, 0x69, 0x67, 0x49, 0x6e, 0x74, 0x52, 0x07, 0x74, 0x6f,
    0x6b, 0x65, 0x6e, 0x49, 0x64, 0x12, 0x23, 0x0a, 0x06, 0x69, 0x74, 0x65, 0x6d, 0x49, 0x64, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x64, 0x63, 0x6c, 0x2e, 0x42, 0x69, 0x67, 0x49,
    0x6e, 0x74, 0x52, 0x06, 0x69, 0x74, 0x65, 0x6d, 0x49, 0x64, 0x12, 0x27, 0x0a, 0x08, 0x69, 0x73,
    0x73, 0x75, 0x65, 0x64, 0x49, 0x64, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x64,
    0x63, 0x6c, 0x2e, 0x42, 0x69, 0x67, 0x49, 0x6e, 0x74, 0x52, 0x08, 0x69, 0x73, 0x73, 0x75, 0x65,
    0x64, 0x49, 0x64, 0x4a, 0xf2, 0x0d, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x33, 0x01, 0x0a, 0x08,
    0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02,
    0x00, 0x0c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x04, 0x00, 0x06, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x04, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x05, 0x04, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x05, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x05, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x05, 0x13,
    0x14, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x08, 0x00, 0x0a, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x08, 0x08, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02,
    0x00, 0x12, 0x03, 0x09, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x09, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x09,
    0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x09, 0x12, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x09, 0x1a, 0x1b, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x0c, 0x00, 0x19, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02,
    0x01, 0x12, 0x03, 0x0c, 0x08, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03,
    0x0d, 0x04, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0d, 0x04,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0d, 0x0b, 0x0d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0d, 0x10, 0x11, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0e, 0x04, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x0e, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x0e, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x0e, 0x14, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x0f, 0x04,
    0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x06, 0x12, 0x03, 0x0f, 0x04, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x0b, 0x15, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0f, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x10, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03,
    0x06, 0x12, 0x03, 0x10, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x10, 0x0b, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x03, 0x12, 0x03, 0x10,
    0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x11, 0x04, 0x15, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x06, 0x12, 0x03, 0x11, 0x04, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x04, 0x01, 0x12, 0x03, 0x11, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x04, 0x03, 0x12, 0x03, 0x11, 0x13, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02,
    0x05, 0x12, 0x03, 0x12, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x05, 0x12,
    0x03, 0x12, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x01, 0x12, 0x03, 0x12,
    0x0b, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x03, 0x12, 0x03, 0x12, 0x19, 0x1a,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x06, 0x12, 0x03, 0x13, 0x04, 0x18, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x06, 0x05, 0x12, 0x03, 0x13, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x06, 0x01, 0x12, 0x03, 0x13, 0x0b, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x06, 0x03, 0x12, 0x03, 0x13, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x07, 0x12,
    0x03, 0x14, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x05, 0x12, 0x03, 0x14,
    0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x01, 0x12, 0x03, 0x14, 0x0b, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x03, 0x12, 0x03, 0x14, 0x1a, 0x1b, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x02, 0x02, 0x08, 0x12, 0x03, 0x15, 0x04, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x08, 0x06, 0x12, 0x03, 0x15, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x08, 0x01, 0x12, 0x03, 0x15, 0x0b, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x08, 0x03,
    0x12, 0x03, 0x15, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x09, 0x12, 0x03, 0x16,
    0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x09, 0x05, 0x12, 0x03, 0x16, 0x04, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x09, 0x01, 0x12, 0x03, 0x16, 0x0b, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x09, 0x03, 0x12, 0x03, 0x16, 0x1b, 0x1d, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x02, 0x02, 0x0a, 0x12, 0x03, 0x17, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x0a, 0x05, 0x12, 0x03, 0x17, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0a, 0x01,
    0x12, 0x03, 0x17, 0x0b, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0a, 0x03, 0x12, 0x03,
    0x17, 0x18, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x0b, 0x12, 0x03, 0x18, 0x04, 0x1a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0b, 0x05, 0x12, 0x03, 0x18, 0x04, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x18, 0x0b, 0x14, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x18, 0x17, 0x19, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03,
    0x12, 0x04, 0x1b, 0x00, 0x1d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x1b,
    0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x1c, 0x04, 0x28, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x1c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x00, 0x06, 0x12, 0x03, 0x1c, 0x0d, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1c, 0x18, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x1c, 0x26, 0x27, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x1f,
    0x00, 0x28, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x1f, 0x08, 0x12, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x20, 0x04, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x20, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x20, 0x0b, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x20, 0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03,
    0x21, 0x04, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x05, 0x12, 0x03, 0x21, 0x04,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x21, 0x0b, 0x0f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x21, 0x12, 0x13, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x04, 0x02, 0x02, 0x12, 0x03, 0x22, 0x04, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x02, 0x05, 0x12, 0x03, 0x22, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x22, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x22, 0x14, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x03, 0x12, 0x03, 0x23, 0x04,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x05, 0x12, 0x03, 0x23, 0x04, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x01, 0x12, 0x03, 0x23, 0x0b, 0x12, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x03, 0x03, 0x12, 0x03, 0x23, 0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x04, 0x02, 0x04, 0x12, 0x03, 0x24, 0x04, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04,
    0x05, 0x12, 0x03, 0x24, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x01, 0x12,
    0x03, 0x24, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x03, 0x12, 0x03, 0x24,
    0x13, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x05, 0x12, 0x03, 0x25, 0x04, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x05, 0x12, 0x03, 0x25, 0x04, 0x08, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x05, 0x01, 0x12, 0x03, 0x25, 0x09, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x05, 0x03, 0x12, 0x03, 0x25, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02,
    0x06, 0x12, 0x03, 0x26, 0x04, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x06, 0x05, 0x12,
    0x03, 0x26, 0x04, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x06, 0x01, 0x12, 0x03, 0x26,
    0x09, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x06, 0x03, 0x12, 0x03, 0x26, 0x17, 0x18,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x07, 0x12, 0x03, 0x27, 0x04, 0x1a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x07, 0x05, 0x12, 0x03, 0x27, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x07, 0x01, 0x12, 0x03, 0x27, 0x0b, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x07, 0x03, 0x12, 0x03, 0x27, 0x18, 0x19, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x2a,
    0x00, 0x2c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x2a, 0x08, 0x0e, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x2b, 0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x03, 0x2b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x00, 0x06, 0x12, 0x03, 0x2b, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x2b, 0x13, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x2b, 0x1c, 0x1d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x2e, 0x00, 0x33, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x2e, 0x08, 0x0d, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x2f, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x2f, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x2f, 0x0b, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x2f, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x01, 0x12, 0x03, 0x30, 0x04, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x06, 0x12, 0x03, 0x30, 0x04, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x01, 0x12, 0x03, 0x30, 0x0b, 0x12, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x01, 0x03, 0x12, 0x03, 0x30, 0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06,
    0x02, 0x02, 0x12, 0x03, 0x31, 0x04, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x06,
    0x12, 0x03, 0x31, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x31, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x03, 0x12, 0x03, 0x31, 0x14,
    0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x03, 0x12, 0x03, 0x32, 0x04, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x06, 0x12, 0x03, 0x32, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x03, 0x01, 0x12, 0x03, 0x32, 0x0b, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x03, 0x03, 0x12, 0x03, 0x32, 0x16, 0x17, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
// @@protoc_insertion_point(module)