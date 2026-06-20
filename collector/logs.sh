#!/bin/bash

# Ensure both start and end arguments are provided
if [ -z "$1" ] || [ -z "$2" ]; then
    echo "Usage: $0 <start_num> <end_num>"
    echo "Example: $0 0 150"
    exit 1
fi

START=$1
END=$2
OUTPUT_FILE="./app.log"

echo "Generating logs from $START to $END into $OUTPUT_FILE..."

# Loop through the specified range
for (( i=START; i<=END; i++ ))
do
    # Get current timestamp with millisecond precision
    TIMESTAMP=$(date +"%Y-%m-%d %H:%M:%S.%3N")
    
    # Append the log entry to the file
    echo "$TIMESTAMP - Log $i" >> "$OUTPUT_FILE"
    
    # Pause for 800 microseconds (0.8 milliseconds)
    # Using perl because it handles sub-millisecond sleeps reliably across systems
    perl -MTime::HiRes=usleep -e 'usleep(2000000)'
done

echo "Done! Generated $((END - START + 1)) log entries."