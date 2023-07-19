-- Declare a variable to store the entity_schema value
DO $$
DECLARE schema_name TEXT;
BEGIN -- Retrieve the entity_schema value using the SELECT query
SELECT information.schema_name INTO schema_name
FROM information_schema.schemata as information
WHERE information.schema_name LIKE 'dcl' || '%'
ORDER BY CAST(
        SUBSTRING(
            information.schema_name
            FROM 'dcl([0-9]+)'
        ) AS INTEGER
    ) desc
LIMIT 1;
-- Set the current working schema using the retrieved entity_schema value
EXECUTE 'SET search_path TO ' || schema_name;
-- Output the current working schema
RAISE NOTICE 'Current working schema set to: %',
schema_name;
END $$;
CREATE TABLE collections (
    id TEXT NOT NULL PRIMARY KEY,
    owner TEXT NOT NULL,
    creator TEXT NOT NULL,
    name TEXT NOT NULL,
    symbol TEXT NOT NULL,
    is_completed BOOLEAN NOT NULL DEFAULT false,
    is_editable BOOLEAN NOT NULL DEFAULT false,
    minters text [],
    managers text [],
    urn text NOT NULL,
    created_at numeric NOT NULL,
    updated_at numeric NOT NULL,
    reviewed_at numeric NOT NULL,
    first_listed_at numeric,
    block_number TEXT NOT NULL
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
    available TEXT NOT NULL,
    price TEXT NOT NULL,
    beneficiary TEXT NOT NULL,
    content_hash text,
    uri text NOT NULL,
    image text,
    minters text [],
    managers text [],
    metadata text,
    raw_metadata TEXT NOT NULL,
    urn text NOT NULL,
    created_at numeric NOT NULL,
    updated_at numeric NOT NULL,
    reviewed_at numeric NOT NULL,
    sold_at numeric,
    first_listed_at numeric,
    block_number TEXT NOT NULL
);
CREATE TABLE metadata (
    id text NOT NULL PRIMARY KEY,
    item_id TEXT NOT NULL,
    item_type TEXT NOT NULL,
    wearable text,
    emote text,
    timestamp TEXT NOT NULL,
    block_number TEXT NOT NULL
);
CREATE TABLE wearable (
    id text NOT NULL PRIMARY KEY,
    metadata text NOT NULL,
    name text NOT NULL,
    description text NOT NULL,
    collection text NOT NULL,
    category text NOT NULL,
    body_shapes text []
);
CREATE TABLE emote (
    id text NOT NULL PRIMARY KEY,
    metadata text NOT NULL,
    name text NOT NULL,
    description text NOT NULL,
    collection text NOT NULL,
    category text NOT NULL,
    loop boolean NOT NULL,
    body_shapes text []
);
CREATE TABLE orders (
    id TEXT NOT NULL PRIMARY KEY,
    marketplace_address TEXT NOT NULL,
    nft TEXT NOT NULL,
    item text,
    nft_address text NOT NULL,
    token_id numeric NOT NULL,
    tx_hash TEXT NOT NULL,
    owner TEXT NOT NULL,
    buyer TEXT,
    price TEXT NOT NULL,
    status TEXT NOT NULL,
    expires_at numeric NOT NULL,
    created_at numeric NOT NULL,
    updated_at numeric NOT NULL,
    block_number TEXT NOT NULL
);
CREATE TABLE nfts (
    id TEXT NOT NULL PRIMARY KEY,
    token_id TEXT NOT NULL,
    collection_id TEXT NOT NULL,
    issued_id TEXT NOT NULL,
    item TEXT NOT NULL,
    owner TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    block_number TEXT NOT NULL
);
CREATE TABLE item_minters (
    id TEXT NOT NULL PRIMARY KEY,
    item_id TEXT NOT NULL,
    minter TEXT NOT NULL,
    value BOOLEAN NOT NULL,
    timestamp TEXT NOT NULL,
    block_number TEXT NOT NULL
);
CREATE TABLE collection_set_approved_events (
    id TEXT NOT NULL PRIMARY KEY,
    collection_id TEXT NOT NULL,
    value BOOLEAN NOT NULL,
    timestamp TEXT NOT NULL,
    block_number TEXT NOT NULL
);
CREATE TABLE collection_set_global_minter_events (
    id TEXT NOT NULL PRIMARY KEY,
    collection_id TEXT NOT NULL,
    minter TEXT NOT NULL,
    value BOOLEAN NOT NULL,
    search_is_store_minter BOOLEAN NOT NULL,
    timestamp TEXT NOT NULL,
    block_number TEXT NOT NULL
);
CREATE TABLE update_item_data_events (
    id TEXT NOT NULL PRIMARY KEY,
    collection_id TEXT NOT NULL,
    item_id TEXT NOT NULL,
    price TEXT NOT NULL,
    beneficiary TEXT NOT NULL,
    raw_metadata TEXT NOT NULL,
    timestamp TEXT NOT NULL,
    block_number TEXT NOT NULL
);