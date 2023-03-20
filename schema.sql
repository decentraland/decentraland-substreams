CREATE TABLE collections (
	id TEXT NOT NULL PRIMARY KEY,
	creator TEXT NOT NULL,
	created_at INTEGER NOT NULL -- created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(), @TODO: do we need timestamp?
	-- updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
create TABLE items (
	id TEXT NOT NULL PRIMARY KEY,
	blockchain_item_id BigInt NOT NULL,
	creator TEXT NOT NULL,
	-- check if needed
	item_type TEXT NOT NULL,
	-- check type
	price TEXT NOT NULL,
	total_supply TEXT NOT NULL,
	available TEXT NOT NULL,
	max_supply TEXT NOT NULL,
	rarity TEXT NOT NULL,
	beneficiary TEXT NOT NULL,
	raw_metadata TEXT NOT NULL,
	collection_id TEXT NOT NULL,
	created_at INTEGER NOT NULL -- created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	-- updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
	-- CONSTRAINT fk_collection FOREIGN KEY(collection_id) REFERENCES collections(id)
);