#! /bin/sh
#
# Download any missing input to input/dayN.txt
# Assumes w3m is already logged into adventofcode.com

set -e

export TZ=US/Eastern

mkdir -p input
event=$(pwd | tail -c 5)

if [ "$event" = "$(date '+%Y')" ] && [ "$(date '+%d')" -le 25 ]; then
    # event in progress
    lastday=$(date '+%d')
else
    # finished event
    lastday=25
fi

for day in $(jot $lastday); do
    input="input/day$day.txt"
    if [ ! -e "$input" ]; then
        url="https://adventofcode.com/$event/day/$day/input"
        echo "Downloading $url -> $input"
        w3m "$url" > "$input"
    fi
done
