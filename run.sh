
#!/bin/bash

# Function to run the CLI command and save the PID to a file
run_process() {
    local command=$1
    local pid_file=$2
    local log_file=$3

    # Run the command in the background, save the PID to the file, and redirect the logs to the specified file
    ($command) > $log_file 2>&1 & echo $! > $pid_file
}

# Prompt the user to enter the network
read -p "Enter the network: (goerli, mainnet or polygon) " network
read -p "Enter the PostgreSQL connection string: " psql_string
read -p "Enter the PostgreSQL schema to sink to: " psql_schema
read -p "Enter the spkg url or yaml: " spkg_string

# Determine the db_out parameter based on the network
if [ "$network" = "polygon" ]; then
    db_out="db_out_polygon"
else
    db_out="db_out"
fi

# Run the process for the specified network
run_process "./substreams-sink-postgres run $psql_string&schema=$psql_schema $network.streamingfast.io:443 $spkg_string $db_out --metrics-listen-addr=0.0.0.0:9102" $network-sink-pid.txt logs-$network.txt &

# Print the PID file for reference
echo "Process PID file: $network-sink-pid.txt"
