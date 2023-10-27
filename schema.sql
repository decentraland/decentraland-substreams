

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
    total_supply numeric NOT NULL,
    max_supply numeric NOT NULL,
    rarity TEXT NOT NULL,
    creation_fee numeric NOT NULL,
    available TEXT NOT NULL,
    price numeric NOT NULL,
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
    timestamp numeric NOT NULL,
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
    body_shapes text [],
    has_sound boolean,
    has_geometry boolean
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
    price numeric NOT NULL, -- update
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
    created_at numeric NOT NULL,
    updated_at numeric NOT NULL,
    block_number numeric NOT NULL
);
CREATE TABLE item_minters (
    id TEXT NOT NULL PRIMARY KEY,
    item_id TEXT NOT NULL,
    minter TEXT NOT NULL,
    value BOOLEAN NOT NULL,
    timestamp numeric NOT NULL,
    block_number numeric NOT NULL
);
CREATE TABLE collection_set_approved_events (
    id TEXT NOT NULL PRIMARY KEY,
    collection_id TEXT NOT NULL,
    value BOOLEAN NOT NULL,
    timestamp numeric NOT NULL,
    block_number numeric NOT NULL
);
CREATE TABLE collection_set_global_minter_events (
    id TEXT NOT NULL PRIMARY KEY,
    collection_id TEXT NOT NULL,
    minter TEXT NOT NULL,
    value BOOLEAN NOT NULL,
    search_is_store_minter BOOLEAN NOT NULL,
    timestamp numeric NOT NULL,
    block_number numeric NOT NULL
);
CREATE TABLE update_item_data_events (
    id TEXT NOT NULL PRIMARY KEY,
    collection_id TEXT NOT NULL,
    item_id TEXT NOT NULL,
    price numeric NOT NULL,
    beneficiary TEXT NOT NULL,
    raw_metadata TEXT NOT NULL,
    timestamp numeric NOT NULL,
    block_number numeric NOT NULL
);
CREATE TABLE transfers (
    id TEXT NOT NULL PRIMARY KEY,
    "to" TEXT NOT NULL,
    "from" TEXT NOT NULL,
    nft TEXT NOT NULL,
    timestamp numeric NOT NULL,
    block_number numeric NOT NULL
);


--- INDEXES

--@TODO ADD INDEXES HERE

CREATE INDEX orders_nft_status_id_created_at_idx ON orders (nft, status, id, created_at DESC);

--- VIEWS, FUNCTIONS & TRIGGERS

--- collection_minters_view
CREATE MATERIALIZED VIEW collection_minters_view AS
SELECT collection_id,
       search_is_store_minter AS is_store_minter,
       timestamp,
       (
                SELECT MIN(timestamp) 
                FROM collection_set_global_minter_events 
                WHERE collection_id = subquery.collection_id AND search_is_store_minter = true
            ) AS first_listed_at
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
    FROM collection_set_global_minter_events
    WHERE (minter = '0x214ffc0f0103735728dc66b61a22e4f163e275ae' OR minter = '0x6ddF1b1924DAD850AdBc1C02026535464Be06B0c')
) AS subquery
WHERE subquery.row_num = 1;

CREATE UNIQUE INDEX idx_collection_minters_view_collection_id
ON collection_minters_view (collection_id);

CREATE OR REPLACE FUNCTION refresh_collection_minters_view()
RETURNS TRIGGER LANGUAGE plpgsql AS
$$
BEGIN
  EXECUTE 'REFRESH MATERIALIZED VIEW CONCURRENTLY ' || TG_TABLE_SCHEMA || '.collection_minters_view';
  RETURN NULL;
END;
$$;

CREATE TRIGGER collection_minters_view_refresh
AFTER INSERT OR UPDATE OR DELETE ON collection_set_global_minter_events
FOR EACH STATEMENT
EXECUTE PROCEDURE refresh_collection_minters_view();

----- nfts_view

CREATE MATERIALIZED VIEW nfts_view AS
SELECT item, COUNT(*) AS nfts_count
FROM nfts
GROUP BY item;

CREATE UNIQUE INDEX idx_nfts_view ON nfts_view (item);

CREATE OR REPLACE FUNCTION refresh_nfts_view()
RETURNS TRIGGER LANGUAGE plpgsql AS
$$
BEGIN
  EXECUTE 'REFRESH MATERIALIZED VIEW CONCURRENTLY ' || TG_TABLE_SCHEMA || '.nfts_view';
  RETURN NULL;
END;
$$;

CREATE TRIGGER nfts_view_refresh
AFTER INSERT OR UPDATE OR DELETE ON nfts
FOR EACH STATEMENT EXECUTE FUNCTION refresh_nfts_view();


----- nfts_with_owners

CREATE MATERIALIZED VIEW nfts_owners_view AS
SELECT item, COUNT(DISTINCT owner) AS owners_count
FROM nfts
GROUP BY item;

CREATE UNIQUE INDEX idx_nfts_owners_view ON nfts_owners_view (item);

CREATE OR REPLACE FUNCTION refresh_nfts_owners_view()
RETURNS TRIGGER LANGUAGE plpgsql AS
$$
BEGIN
  EXECUTE 'REFRESH MATERIALIZED VIEW CONCURRENTLY ' || TG_TABLE_SCHEMA || '.nfts_owners_view';
  RETURN NULL;
END;
$$;

CREATE TRIGGER nfts_owners_view_refresh
AFTER INSERT OR UPDATE OR DELETE ON nfts
FOR EACH STATEMENT EXECUTE FUNCTION refresh_nfts_owners_view();

CREATE TRIGGER transfers_nfts_owners_view_refresh
AFTER INSERT OR UPDATE OR DELETE ON transfers
FOR EACH STATEMENT EXECUTE FUNCTION refresh_nfts_owners_view();

---- latest prices view

CREATE MATERIALIZED VIEW latest_prices_view AS
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
    FROM update_item_data_events
) AS subquery
WHERE subquery.row_num = 1;

CREATE UNIQUE INDEX idx_latest_prices_view_item_id
ON latest_prices_view (item_id);

CREATE OR REPLACE FUNCTION refresh_latest_prices_view()
RETURNS TRIGGER LANGUAGE plpgsql AS
$$
BEGIN
  EXECUTE 'REFRESH MATERIALIZED VIEW CONCURRENTLY ' || TG_TABLE_SCHEMA || '.latest_prices_view';
  RETURN NULL;
END;
$$;

CREATE TRIGGER latest_prices_view_refresh
AFTER INSERT OR UPDATE OR DELETE
ON update_item_data_events
FOR EACH STATEMENT
EXECUTE FUNCTION refresh_latest_prices_view();

--- orders

CREATE MATERIALIZED VIEW nfts_with_orders_view AS
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
    FROM orders AS orders
    WHERE orders.status = 'open'
        AND orders.expires_at < 253378408747000
        AND ((LENGTH(orders.expires_at::text) = 13 AND TO_TIMESTAMP(orders.expires_at / 1000.0) > NOW())
                OR
            (LENGTH(orders.expires_at::text) = 10 AND TO_TIMESTAMP(orders.expires_at) > NOW()))
) AS nfts_with_orders
GROUP BY nfts_with_orders.item;

CREATE UNIQUE INDEX idx_nfts_with_orders_view
ON nfts_with_orders_view (item);

CREATE OR REPLACE FUNCTION refresh_nfts_with_orders_view()
RETURNS TRIGGER LANGUAGE plpgsql AS
$$
BEGIN
  EXECUTE 'REFRESH MATERIALIZED VIEW CONCURRENTLY ' || TG_TABLE_SCHEMA || '.nfts_with_orders_view';
  RETURN NULL;
END;
$$;

CREATE TRIGGER nfts_with_orders_view_refresh
AFTER INSERT OR UPDATE OR DELETE
ON orders
FOR EACH STATEMENT
EXECUTE FUNCTION refresh_nfts_with_orders_view();

--- update older orders on insert

CREATE OR REPLACE FUNCTION update_old_orders()
RETURNS TRIGGER AS $$
DECLARE
  recent_order_id TEXT;
BEGIN

  EXECUTE format('SELECT id FROM %I.orders WHERE nft = $1 AND status = ''open'' AND id <> $2 ORDER BY created_at DESC LIMIT 1', TG_TABLE_SCHEMA)
  INTO recent_order_id
  USING NEW.nft, NEW.id;

  IF recent_order_id IS NOT NULL THEN
    EXECUTE format('UPDATE %I.orders SET status = ''cancelled'', updated_at = $1 WHERE id = $2', TG_TABLE_SCHEMA)
    USING NEW.created_at, recent_order_id;
  END IF;

  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_old_orders
AFTER INSERT ON orders
FOR EACH ROW EXECUTE FUNCTION update_old_orders();

---- cancel orders on transfer

CREATE OR REPLACE FUNCTION cancel_orders_on_transfer()
RETURNS TRIGGER LANGUAGE plpgsql AS
$$
BEGIN
  EXECUTE format('UPDATE %I.orders SET status = ''cancelled'', updated_at = $1 WHERE nft = $2 AND owner = $3 AND status = ''open''', TG_TABLE_SCHEMA)
  USING NEW.timestamp::integer, NEW.nft, NEW."from";

  RETURN NEW;
END;
$$;

CREATE TRIGGER cancel_orders_on_transfer_trigger
AFTER INSERT ON transfers
FOR EACH ROW EXECUTE FUNCTION cancel_orders_on_transfer();

------ item_set_minter_event_view

CREATE MATERIALIZED VIEW item_set_minter_event_view AS
WITH NumberedEvents AS (
    SELECT
        item_id,
        value,
        timestamp,
        ROW_NUMBER() OVER (
          PARTITION BY item_id
          ORDER BY timestamp DESC
        ) AS row_num
    FROM
        item_minters
    WHERE
        minter = '0x214ffc0f0103735728dc66b61a22e4f163e275ae' OR minter = '0x6ddF1b1924DAD850AdBc1C02026535464Be06B0c'
)
SELECT 
    item_id,
    value,
    timestamp
FROM
    NumberedEvents
WHERE
    row_num = 1;

-- Create the unique index for the materialized view
CREATE UNIQUE INDEX idx_item_set_minter_event_view_item_id
ON item_set_minter_event_view (item_id);

-- Create the function to refresh the materialized view
CREATE OR REPLACE FUNCTION refresh_item_set_minter_event_view()
RETURNS TRIGGER LANGUAGE plpgsql AS
$$
BEGIN
  EXECUTE 'REFRESH MATERIALIZED VIEW CONCURRENTLY ' || TG_TABLE_SCHEMA || '.item_set_minter_event_view';
  RETURN NULL;
END;
$$;

-- Create the trigger to refresh the materialized view when there are changes to the underlying table
CREATE TRIGGER item_set_minter_event_view_refresh
AFTER INSERT OR UPDATE OR DELETE
ON item_minters
FOR EACH STATEMENT
EXECUTE FUNCTION refresh_item_set_minter_event_view();

-----

CREATE MATERIALIZED VIEW collection_set_approved_events_view AS
WITH NumberedEvents AS (
    SELECT
        collection_id,
        value,
        timestamp,
        ROW_NUMBER() OVER (
          PARTITION BY collection_id
          ORDER BY timestamp DESC
        ) AS row_num
    FROM
        collection_set_approved_events
    WHERE
        value = true
)
SELECT 
    collection_id,
    value,
    timestamp
FROM
    NumberedEvents
WHERE
    row_num = 1;

-- Create the unique index for the materialized view
CREATE UNIQUE INDEX idx_collection_set_approved_events_view_collection_id
ON collection_set_approved_events_view (collection_id);

-- Create the function to refresh the materialized view
CREATE OR REPLACE FUNCTION refresh_collection_set_approved_events_view()
RETURNS TRIGGER LANGUAGE plpgsql AS
$$
BEGIN
  EXECUTE 'REFRESH MATERIALIZED VIEW CONCURRENTLY ' || TG_TABLE_SCHEMA || '.collection_set_approved_events_view';
  RETURN NULL;
END;
$$;

-- Create the trigger to refresh the materialized view when there are changes to the underlying table
CREATE TRIGGER collection_set_approved_events_view_refresh
AFTER INSERT OR UPDATE OR DELETE
ON collection_set_approved_events
FOR EACH STATEMENT
EXECUTE FUNCTION refresh_collection_set_approved_events_view();
