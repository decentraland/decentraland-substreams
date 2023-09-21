SET search_path TO dcl24;

--- refresh_collection_minters_view

CREATE TRIGGER collection_minters_view_refresh
AFTER INSERT OR UPDATE OR DELETE ON collection_set_global_minter_events
FOR EACH STATEMENT
EXECUTE PROCEDURE refresh_collection_minters_view();

--- collection_set_approved_events_view_refresh

CREATE TRIGGER collection_set_approved_events_view_refresh
AFTER INSERT OR UPDATE OR DELETE
ON collection_set_approved_events
FOR EACH STATEMENT
EXECUTE FUNCTION refresh_collection_set_approved_events_view();

--- item_set_minter_event_view
CREATE TRIGGER item_set_minter_event_view_refresh
AFTER INSERT OR UPDATE OR DELETE
ON item_minters
FOR EACH STATEMENT
EXECUTE FUNCTION refresh_item_set_minter_event_view();

--- latest_prices_view

CREATE TRIGGER latest_prices_view_refresh
AFTER INSERT OR UPDATE OR DELETE
ON update_item_data_events
FOR EACH STATEMENT
EXECUTE FUNCTION refresh_latest_prices_view();

--- nfts_owners_view

CREATE TRIGGER nfts_owners_view_refresh
AFTER INSERT OR UPDATE OR DELETE ON nfts
FOR EACH STATEMENT EXECUTE FUNCTION refresh_nfts_owners_view();

CREATE TRIGGER transfers_nfts_owners_view_refresh
AFTER INSERT OR UPDATE OR DELETE ON transfers
FOR EACH STATEMENT EXECUTE FUNCTION refresh_nfts_owners_view();

--- nfts_view

CREATE TRIGGER nfts_view_refresh
AFTER INSERT OR UPDATE OR DELETE ON nfts
FOR EACH STATEMENT EXECUTE FUNCTION refresh_nfts_view();

--- nfts_with_orders_view

CREATE TRIGGER nfts_with_orders_view_refresh
AFTER INSERT OR UPDATE OR DELETE
ON orders
FOR EACH STATEMENT
EXECUTE FUNCTION refresh_nfts_with_orders_view();