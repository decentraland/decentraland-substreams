-- -- Creation of product table
-- CREATE TABLE IF NOT EXISTS product (
--   product_id INT NOT NULL,
--   name varchar(250) NOT NULL,
--   PRIMARY KEY (product_id)
-- );

CREATE TABLE IF NOT EXISTS cursors (
    id         TEXT NOT NULL CONSTRAINT cursor_pk PRIMARY KEY,
    cursor     TEXT,
    block_num  BigInt,
    block_id   TEXT
);

CREATE TABLE IF NOT EXISTS block_meta (
    id          TEXT NOT NULL CONSTRAINT block_meta_pk PRIMARY KEY,
    at          TIMESTAMP,
    number      BigInt,
    hash        TEXT,
    parent_hash TEXT,
    timestamp   TIMESTAMP
);

CREATE TABLE IF NOT EXISTS collections (
	id TEXT NOT NULL PRIMARY KEY,
	creator TEXT NOT NULL,
	is_approved BOOLEAN NOT NULL DEFAULT false,
	name TEXT NOT NULL,
	symbol TEXT NOT NULL,
	owner TEXT NOT NULL,
	is_completed BOOLEAN NOT NULL DEFAULT false,
	created_at INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS items (
	id TEXT NOT NULL PRIMARY KEY,
	blockchain_item_id BigInt NOT NULL,
	item_type TEXT NOT NULL,
	price TEXT NOT NULL,
	total_supply TEXT NOT NULL,
	available TEXT NOT NULL,
	max_supply TEXT NOT NULL,
	rarity TEXT NOT NULL,
	beneficiary TEXT NOT NULL,
	raw_metadata TEXT NOT NULL,
	collection_id TEXT NOT NULL,
	created_at INTEGER NOT NULL -- CONSTRAINT fk_collection FOREIGN KEY(collection_id) REFERENCES collections(id)
);

CREATE TABLE IF NOT EXISTS orders (
	id TEXT NOT NULL PRIMARY KEY,
	marketplace_address TEXT NOT NULL,
	nft_id TEXT NOT NULL,
	token_id TEXT NOT NULL,
	tx_hash TEXT NOT NULL,
	owner TEXT NOT NULL,
	buyer TEXT,
	price TEXT NOT NULL,
	status TEXT NOT NULL,
	block_number TEXT NOT NULL,
	expires_at TEXT NOT NULL,
	updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS nfts (
	id TEXT NOT NULL PRIMARY KEY,
	token_id TEXT NOT NULL,
	collection_id TEXT NOT NULL, -- CONSTRAINT fk_collection FOREIGN KEY(collection_id) REFERENCES collections(id)
	issued_id TEXT NOT NULL,
	item_id TEXT NOT NULL,
	owner TEXT NOT NULL,
	created_at TEXT NOT NULL,
	updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS transfers (
	id TEXT NOT NULL PRIMARY KEY,
	token_id TEXT NOT NULL, -- CONSTRAINT fk_nfts FOREIGN KEY(token_id) REFERENCES nfts(token_id)
	collection_id TEXT NOT NULL, -- CONSTRAINT fk_collection FOREIGN KEY(collection_id) REFERENCES collections(id)
	block_timestamp INTEGER NOT NULL,
	from_address TEXT NOT NULL,
	to_address TEXT NOT NULL
);