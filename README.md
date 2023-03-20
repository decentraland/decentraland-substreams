# decentraland-substreams

An implementation of StreamingFast's substreams for dcl contracts

# Substreams

> Developer preview

Substreams is a powerful blockchain indexing technology, developed for The Graph Network.

Substreams enables developers to write Rust modules, composing data streams alongside the community, and provides extremely high performance indexing by virtue of parallelization, in a streaming-first fashion.

Substreams has all the benefits of StreamingFast Firehose, like low-cost caching and archiving of blockchain data, high throughput processing, and cursor-based reorgs handling.

# Running the substreams

First, compile the substreams modules:

```
cargo build --release --target wasm32-unknown-unknown
```

Then, run the module that you want:

```
substreams run -e mainnet.eth.streamingfast.io:443 substreams.yaml map_collection_created --start-block 10000001 --stop-block +1
```


# Running the sink module

You'll need to have the [substreams-sink-postgres](https://github.com/streamingfast/substreams-sink-postgres) cli installed. I recommend using the `go` installation: 
```
go install github.com/streamingfast/substreams-sink-postgres/cmd/substreams-sink-postgres@latest.
```
Check if you have the GOPATH as part of your `PATH`. If not, add it:
```
export PATH=$PATH:$(go env GOPATH)/bin
```
Finally, run the sink 
```
substreams-sink-postgres run \    
    "psql://db_user:db_pass@localhost:5432/db_name?sslmode=disable" \
    "polygon.streamingfast.io:443" \
    "substreams.yaml" \
    db_out
```    

