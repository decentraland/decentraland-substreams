#!/bin/bash

# Function to determine the current operating system
get_os() {
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        echo "linux"
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        echo "macos"
    else
        echo "unsupported"
    fi
}

# Step 1: Determine the operating system and set the appropriate download URL
OS=$(get_os)
if [ "$OS" == "linux" ]; then
    URL="https://github.com/streamingfast/substreams-sink-postgres/releases/download/v2.3.3/substreams-sink-postgres_linux_x86_64.tar.gz"
elif [ "$OS" == "macos" ]; then
    URL="https://github.com/streamingfast/substreams-sink-postgres/releases/download/v2.3.3/substreams-sink-postgres_darwin_x86_64.tar.gz"
else
    echo "Unsupported operating system"
    exit 1
fi

# Step 2: Download and extract the tar.gz file
curl -L $URL | tar -xz

# Step 3: Make the binary executable
chmod +x substreams-sink-postgres
