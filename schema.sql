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
    block_number numeric NOT NULL
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
    block_number numeric NOT NULL
);
CREATE TABLE metadata (
    id text NOT NULL PRIMARY KEY,
    item_id TEXT NOT NULL,
    item_type TEXT NOT NULL,
    wearable text,
    emote text,
    timestamp TEXT NOT NULL,
    block_number numeric NOT NULL
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
    block_number numeric NOT NULL
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
    block_number numeric NOT NULL
);
CREATE TABLE item_minters (
    id TEXT NOT NULL PRIMARY KEY,
    item_id TEXT NOT NULL,
    minter TEXT NOT NULL,
    value BOOLEAN NOT NULL,
    timestamp TEXT NOT NULL,
    block_number numeric NOT NULL
);
CREATE TABLE collection_set_approved_events (
    id TEXT NOT NULL PRIMARY KEY,
    collection_id TEXT NOT NULL,
    value BOOLEAN NOT NULL,
    timestamp TEXT NOT NULL,
    block_number numeric NOT NULL
);
CREATE TABLE collection_set_global_minter_events (
    id TEXT NOT NULL PRIMARY KEY,
    collection_id TEXT NOT NULL,
    minter TEXT NOT NULL,
    value BOOLEAN NOT NULL,
    search_is_store_minter BOOLEAN NOT NULL,
    timestamp TEXT NOT NULL,
    block_number numeric NOT NULL
);
CREATE TABLE update_item_data_events (
    id TEXT NOT NULL PRIMARY KEY,
    collection_id TEXT NOT NULL,
    item_id TEXT NOT NULL,
    price TEXT NOT NULL,
    beneficiary TEXT NOT NULL,
    raw_metadata TEXT NOT NULL,
    timestamp TEXT NOT NULL,
    block_number numeric NOT NULL
);


--- INDEXES

--@TODO ADD INDEXES HERE

--- VIEWS, FUNCTIONS & TRIGGERS

CREATE MATERIALIZED VIEW dcl11.collection_minters_view AS
SELECT collection_id,
       search_is_store_minter AS is_store_minter,
       timestamp
FROM (
    SELECT collection_id,
           value,
           minter,
           timestamp,
           search_is_store_minter,
           ROW_NUMBER() OVER (
               PARTITION BY collection_id
               ORDER BY timestamp DESC
           ) AS row_num
    FROM dcl11.collection_set_global_minter_events
    WHERE minter = '0x214ffc0f0103735728dc66b61a22e4f163e275ae' 
) AS subquery
WHERE subquery.row_num = 1;

CREATE UNIQUE INDEX idx_collection_minters_view_collection_id
ON dcl11.collection_minters_view (collection_id);

CREATE OR REPLACE FUNCTION refresh_collection_minters_view()
RETURNS TRIGGER LANGUAGE plpgsql AS $$
BEGIN
  REFRESH MATERIALIZED VIEW CONCURRENTLY dcl11.collection_minters_view;
  RETURN NEW;
END;
$$;

CREATE TRIGGER collection_minters_view_refresh
AFTER INSERT OR UPDATE OR DELETE ON dcl11.collection_set_global_minter_events
FOR EACH STATEMENT
EXECUTE PROCEDURE refresh_collection_minters_view();

----- nfts_view

CREATE MATERIALIZED VIEW dcl11.nfts_view AS
SELECT item, COUNT(*) AS nfts_count
FROM dcl11.nfts
GROUP BY item;

CREATE INDEX idx_nfts_view
ON dcl11.nfts_view (item);

CREATE OR REPLACE FUNCTION dcl11.refresh_nfts_view()
RETURNS TRIGGER LANGUAGE plpgsql AS $$
BEGIN
  REFRESH MATERIALIZED VIEW dcl11.nfts_view;
  RETURN NULL;
END $$;

CREATE TRIGGER nfts_view_refresh
AFTER INSERT OR UPDATE OR DELETE ON dcl11.nfts
FOR EACH STATEMENT EXECUTE FUNCTION dcl11.refresh_nfts_view();


---- latest prices view

CREATE MATERIALIZED VIEW dcl11.latest_prices_view AS
SELECT item_id,
       price,
       timestamp
FROM (
    SELECT item_id,
           price,
           timestamp,
           ROW_NUMBER() OVER (
               PARTITION BY item_id
               ORDER BY timestamp DESC
           ) AS row_num
    FROM dcl11.update_item_data_events
) AS subquery
WHERE subquery.row_num = 1;

CREATE UNIQUE INDEX idx_latest_prices_view_item_id
ON dcl11.latest_prices_view (item_id);

CREATE OR REPLACE FUNCTION dcl11.refresh_latest_prices_view()
RETURNS TRIGGER LANGUAGE plpgsql AS $$
BEGIN
    REFRESH MATERIALIZED VIEW dcl11.latest_prices_view;
    RETURN NULL;
END $$;

CREATE TRIGGER latest_prices_view_refresh
AFTER INSERT OR UPDATE OR DELETE
ON dcl11.update_item_data_events
FOR EACH STATEMENT
EXECUTE FUNCTION dcl11.refresh_latest_prices_view();

--- orders

CREATE MATERIALIZED VIEW dcl11.nfts_with_orders_view AS
SELECT nfts_with_orders.item,
       COUNT(nfts_with_orders.id) AS listings_count,
       MIN(nfts_with_orders.price) AS min_price,
       MAX(nfts_with_orders.price) AS max_price,
       MAX(nfts_with_orders.created_at) AS max_order_created_at
FROM (
    SELECT orders.item,
           orders.id,
           orders.price,
           orders.created_at,
           ROW_NUMBER() OVER (
               PARTITION BY orders.item
               ORDER BY orders.created_at DESC
           ) AS row_num
    FROM dcl11.orders AS orders
    WHERE orders.status = 'open'
        AND orders.expires_at < 253378408747000
        AND to_timestamp(orders.expires_at / 1000.0) > now()
) AS nfts_with_orders
WHERE nfts_with_orders.row_num = 1
GROUP BY nfts_with_orders.item;


CREATE UNIQUE INDEX idx_nfts_with_orders_view
ON dcl11.nfts_with_orders_view (item);

CREATE OR REPLACE FUNCTION dcl11.refresh_nfts_with_orders_view()
RETURNS TRIGGER LANGUAGE plpgsql AS
$$
BEGIN
  REFRESH MATERIALIZED VIEW CONCURRENTLY dcl11.nfts_with_orders_view;
  RETURN NULL;
END;
$$;

CREATE TRIGGER nfts_with_orders_view_refresh
AFTER INSERT OR UPDATE OR DELETE
ON dcl11.orders
FOR EACH STATEMENT
EXECUTE FUNCTION dcl11.refresh_nfts_with_orders_view();

