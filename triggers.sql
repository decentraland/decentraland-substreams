-- IMPORTANT: SET THE RIGHT SCHEMA HERE BEFORE RUNNING
SET search_path TO dcl32;

--- VIEWS

--- collection_minters_view
CREATE MATERIALIZED VIEW collection_minters_view AS
SELECT 
	minter,
    collection_id,
    search_is_store_minter AS is_store_minter,
    timestamp,
    (
        SELECT MIN(timestamp) 
        FROM collection_set_global_minter_events 
        WHERE collection_id = subquery.collection_id AND search_is_store_minter = true
    ) AS first_listed_at
FROM (
    SELECT 
    	minter,
        collection_id,
        timestamp,
        -- Calculate search_is_store_minter based on the minter's address
        CASE
            WHEN minter IN ('0x214ffc0f0103735728dc66b61a22e4f163e275ae', '0xe36abc9ec616c83caaa386541380829106149d68') THEN true
            ELSE false
        END AS search_is_store_minter,
        ROW_NUMBER() OVER (
            PARTITION BY collection_id
            ORDER BY timestamp DESC
        ) AS row_num
    FROM collection_set_global_minter_events
    WHERE minter IN ('0x214ffc0f0103735728dc66b61a22e4f163e275ae', '0xe36abc9ec616c83caaa386541380829106149d68')
) AS subquery
WHERE subquery.row_num = 1;

CREATE UNIQUE INDEX idx_collection_minters_view_collection_id
ON collection_minters_view (collection_id);

----- nfts_view

CREATE MATERIALIZED VIEW nfts_view AS
SELECT item, COUNT(*) AS nfts_count
FROM nfts
GROUP BY item;

CREATE UNIQUE INDEX idx_nfts_view ON nfts_view (item);


----- nfts_with_owners

CREATE MATERIALIZED VIEW nfts_owners_view AS
SELECT item, COUNT(DISTINCT owner) AS owners_count
FROM nfts
GROUP BY item;

CREATE UNIQUE INDEX idx_nfts_owners_view ON nfts_owners_view (item);

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

CREATE UNIQUE INDEX idx_item_set_minter_event_view_item_id
ON item_set_minter_event_view (item_id);


----- collection_set_approved_events_view

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
    -- WHERE
    --     value = true
)
SELECT 
    collection_id,
    value,
    timestamp
FROM
    NumberedEvents
WHERE
    row_num = 1;

----- item_sold_at_view

CREATE MATERIALIZED VIEW item_sold_at_view AS
SELECT item, MAX(updated_at) from orders WHERE status = 'sold'
GROUP BY orders.item;

CREATE UNIQUE INDEX idx_item_sold_at_view_item
ON item_sold_at_view (item);

-- FUNCTIONS && TRIGGERS

-- Function for refreshing materialized views
CREATE OR REPLACE FUNCTION refresh_mat_view()
RETURNS TRIGGER LANGUAGE plpgsql AS
$$
BEGIN
	PERFORM pg_notify('refresh_mat_view', TG_TABLE_SCHEMA || '.' || TG_ARGV[0]);
	RETURN NULL;
END;
$$;

CREATE TRIGGER collection_minters_view_refresh
AFTER INSERT OR UPDATE OR DELETE ON collection_set_global_minter_events
FOR EACH STATEMENT
EXECUTE FUNCTION refresh_mat_view('collection_minters_view');

CREATE TRIGGER nfts_view_refresh
AFTER INSERT OR UPDATE OR DELETE ON nfts
FOR EACH STATEMENT
EXECUTE FUNCTION refresh_mat_view('nfts_view');

CREATE TRIGGER nfts_owners_view_refresh
AFTER INSERT OR UPDATE OR DELETE ON nfts
FOR EACH STATEMENT
EXECUTE FUNCTION refresh_mat_view('nfts_owners_view');

CREATE TRIGGER transfers_nfts_owners_view_refresh
AFTER INSERT OR UPDATE OR DELETE ON transfers
FOR EACH STATEMENT
EXECUTE FUNCTION refresh_mat_view('nfts_owners_view');

CREATE TRIGGER latest_prices_view_refresh
AFTER INSERT OR UPDATE OR DELETE ON update_item_data_events
FOR EACH STATEMENT
EXECUTE FUNCTION refresh_mat_view('latest_prices_view');

CREATE TRIGGER nfts_with_orders_view_refresh
AFTER INSERT OR UPDATE OR DELETE ON orders
FOR EACH STATEMENT
EXECUTE FUNCTION refresh_mat_view('nfts_with_orders_view');

CREATE TRIGGER item_set_minter_event_view_refresh
AFTER INSERT OR UPDATE OR DELETE ON item_minters
FOR EACH STATEMENT
EXECUTE FUNCTION refresh_mat_view('item_set_minter_event_view');

CREATE TRIGGER item_sold_at_view_refresh
AFTER INSERT OR UPDATE OR DELETE ON orders
FOR EACH STATEMENT
EXECUTE FUNCTION refresh_mat_view('item_sold_at_view');

CREATE TRIGGER collection_set_approved_events_view_refresh
AFTER INSERT OR UPDATE OR DELETE ON collection_set_approved_events
FOR EACH STATEMENT
EXECUTE FUNCTION refresh_mat_view('collection_set_approved_events_view');
