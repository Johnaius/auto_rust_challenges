#!/bin/bash
if [ -z "$1" ]; then
    echo "Usage: $0 <day_number>"
    exit 1
fi

DAY="day$1"
cd "dailypractice/$DAY" || { echo "Directory $DAY not found"; exit 1; }
cargo run