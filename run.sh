#!/bin/bash

# Function to run the CLI command and save the PID to a file
run_process() {
    local command=$1
    local pid_file=$2
    local log_file=$3

    # Run the command in the background, save the PID to the file, and redirect the logs to the specified file
    ($command) > $log_file 2>&1 & echo $! > $pid_file
}

# Initialize variables for optional flags
on_module_hash_mismatch=""
undo_buffer_size=""

# Parse command-line arguments for flags
for arg in "$@"; do
  case $arg in
    --on-module-hash-mistmatch=*) on_module_hash_mismatch="${arg#*=}" ;;
    --undo-buffer-size=*) undo_buffer_size="${arg#*=}" ;;
  esac
done

# Read the PostgreSQL connection string from the environment variable
psql_string=$SUBSTREAMS_DATABASE

# Prompt the user to enter the network
read -p "Enter the network: (mainnet, sepolia, polygon or mumbai) " network
read -p "Enter the PostgreSQL schema to sink to: " psql_schema

# Construct the yaml
yaml="./substreams-$network.yaml"

# Set the prometheus_port and db_out based on the network
case $network in
    "polygon")
        prometheus_port="9102"
        ;;
    "mainnet")
        prometheus_port="9103"
        ;;
    "sepolia")
        prometheus_port="9104"
        ;;
    "mumbai")
        prometheus_port="9105"
        ;;
    *)
        echo "Invalid network provided!"
        exit 1
        ;;
esac

# Construct the base run command
run_command="./substreams-sink-postgres run $psql_string&schema=$psql_schema $yaml --metrics-listen-addr=0.0.0.0:$prometheus_port"

# Add the undo buffer size if provided
echo $undo_buffer_size
if [ -n "$undo_buffer_size" ]; then
    run_command+=" --undo-buffer-size=$undo_buffer_size"
fi



# Add the on-module-hash-mismatch flag if provided
if [ -n "$on_module_hash_mismatch" ]; then
    run_command+=" --on-module-hash-mistmatch=$on_module_hash_mismatch"
fi

echo $run_command

# Run the process for the specified network
run_process "$run_command" "$network-sink-pid.txt" "logs-$network.txt" &

# Print the Prometheus port for reference
echo "Prometheus metrics exported on port: $prometheus_port"

# Print the PID file for reference
echo "Process PID file: $network-sink-pid.txt"
