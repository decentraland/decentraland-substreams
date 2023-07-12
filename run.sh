
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

# Set the prometheus_port based on the network
case $network in
    "polygon")
        prometheus_port="9102"
        ;;
    "mainnet")
        prometheus_port="9103"
        ;;
    "goerli")
        prometheus_port="9104"
        ;;
    *)
        echo "Invalid network provided!"
        exit 1
        ;;
esac

# Determine the db_out parameter based on the network
if [ "$network" = "polygon" ]; then
    db_out="db_out_polygon"
else
    db_out="db_out"
fi

# Run the process for the specified network
run_process "./substreams-sink-postgres run $psql_string&schema=$psql_schema $network.streamingfast.io:443 $spkg_string $db_out --metrics-listen-addr=0.0.0.0:$prometheus_port" $network-sink-pid.txt logs-$network.txt &

# Print the Prometheus port for reference
echo "Prometheus metrics exported on port: $prometheus_port"

# Print the PID file for reference
echo "Process PID file: $network-sink-pid.txt"
