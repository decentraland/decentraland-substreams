# #!/bin/bash

# # Run the first process (ethereum-mainnet) as a daemon
# nohup ./substreams-sink-postgres run "psql://dev-node:insecure-change-me-in-prod@localhost:5432/substreams-example?sslmode=disable" "mainnet.eth.streamingfast.io:443" "./docs/tutorial/substreams.yaml" db_out --metrics-listen-addr=0.0.0.0:9102 2> logs-ethereum-mainnet.txt > /dev/null &

# # Run the second process (ethereum-goerli) as a daemon
# nohup ./substreams-sink-postgres run "psql://dev-node:insecure-change-me-in-prod@localhost:5432/substreams-example?sslmode=disable" "goerli.eth.streamingfast.io:443" "./docs/tutorial/substreams.yaml" db_out --metrics-listen-addr=0.0.0.0:9103 2> logs-ethereum-goerli.txt > /dev/null &

# # Run the third process (polygon-mainnet) as a daemon
# nohup ./substreams-sink-postgres run "psql://dev-node:insecure-change-me-in-prod@localhost:5432/substreams-example?sslmode=disable" "polygon.streamingfast.io:443" "./docs/tutorial/substreams.yaml" db_out --metrics-listen-addr=0.0.0.0:9104 2> logs-polygon-mainnet.txt > /dev/null &


#!/bin/bash

# Function to run the CLI command and save the PID to a file
run_process() {
    local command=$1
    local pid_file=$2
    local log_file=$3

    # Run the command in the background, save the PID to the file, and redirect the logs to the specified file
    ($command) > $log_file 2>&1 & echo $! > $pid_file
}

# Step 1: Download Go (if not already installed)

# Step 2: Download the CLI using `go install`
# go install github.com/streamingfast/substreams-sink-postgres/cmd/substreams-sink-postgres@latest

# Prompt the user to enter the network
read -p "Enter the network: (goerli, mainnet or polygon) " network
read -p "Enter the PostgreSQL connection string: " psql_string
read -p "Enter the PostgreSQL schema to sink to: " psql_schema

# Determine the db_out parameter based on the network
if [ "$network" = "polygon" ]; then
    db_out="db_out_polygon"
else
    db_out="db_out"
fi

# Run the process for the specified network
run_process "./substreams-sink-postgres run $psql_string&schema=$psql_schema $network.streamingfast.io:443 substreams-$network.yaml $db_out --metrics-listen-addr=0.0.0.0:9102" process-$network.pid logs-$network.txt &

# Print the PID file for reference
echo "Process PID file: process.pid"
