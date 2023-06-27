CREATE TABLE collections (
    id TEXT NOT NULL PRIMARY KEY,
    owner TEXT NOT NULL,
    creator TEXT NOT NULL,
    name TEXT NOT NULL,
    symbol TEXT NOT NULL,
    is_completed BOOLEAN NOT NULL DEFAULT false,
    is_approved BOOLEAN NOT NULL DEFAULT false,
    is_editable BOOLEAN NOT NULL DEFAULT false,
    minters text [],
    managers text [],
    urn text NOT NULL,
    created_at numeric NOT NULL,
    updated_at numeric NOT NULL,
    reviewed_at numeric NOT NULL,
    first_listed_at numeric,
    search_is_store_minter boolean NOT NULL
);
create TABLE items (
    id TEXT NOT NULL PRIMARY KEY,
    collection TEXT NOT NULL,
    blockchain_id BigInt NOT NULL,
    creator text NOT NULL,
    item_type TEXT NOT NULL,
    total_supply TEXT NOT NULL,
    max_supply TEXT NOT NULL,
    rarity TEXT NOT NULL,
    creation_fee numeric NOT NULL,
    available numeric NOT NULL,
    price TEXT NOT NULL,
    beneficiary TEXT NOT NULL,
    content_hash text,
    uri text NOT NULL,
    image text,
    minters text[], -- update to NOT NULL later on
    managers text[], -- update to NOT NULL later on
    metadata text,
    raw_metadata TEXT NOT NULL,
    urn text NOT NULL,
    created_at numeric NOT NULL,
    updated_at numeric NOT NULL,
    reviewed_at numeric NOT NULL,
    sold_at numeric,
    first_listed_at numeric,
    search_is_store_minter boolean NOT NULL,
    search_is_collection_approved boolean
);
CREATE TABLE metadata (
    id text NOT NULL,
    item_type TEXT NOT NULL,
    wearable text,
    emote text
);
CREATE TABLE wearable (
    id text NOT NULL,
    name text NOT NULL,
    description text NOT NULL,
    collection text NOT NULL,
    category text NOT NULL,
    rarity text NOT NULL,
    body_shapes text []
);
CREATE TABLE emote (
    id text NOT NULL,
    name text NOT NULL,
    description text NOT NULL,
    collection text NOT NULL,
    category text NOT NULL,
    loop boolean NOT NULL,
    rarity text NOT NULL,
    body_shapes text []
);
CREATE TABLE orders (
    id TEXT NOT NULL PRIMARY KEY,
    marketplace_address TEXT NOT NULL,
    nft TEXT NOT NULL,
    item text, -- 
    nft_address text NOT NULL, --
    token_id numeric NOT NULL,
    tx_hash TEXT NOT NULL,
    owner TEXT NOT NULL,
    buyer TEXT,
    price TEXT NOT NULL,
    status TEXT NOT NULL,
    block_number TEXT NOT NULL,
    expires_at numeric NOT NULL,
    created_at numeric NOT NULL,
    updated_at numeric NOT NULL
);
CREATE TABLE nfts (
    id TEXT NOT NULL PRIMARY KEY,
    token_id TEXT NOT NULL,
    collection_id TEXT NOT NULL,
    issued_id TEXT NOT NULL,
    item TEXT NOT NULL,
    owner TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);