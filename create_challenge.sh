#!/bin/bash
if [ -z "$1" ]; then
    echo "Usage: $0 <day_number>"
    exit 1
fi

DAY="day$1"
cp -r template "dailypractice/$DAY"
mv "dailypractice/$DAY/template" "practice/$DAY/$DAY"
echo "Challenge setup for $DAY completed."