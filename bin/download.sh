#! /bin/sh
#
# Download any missing input to input/YYYY/dayN.txt
# Cookie grabbed from firefox, must already be logged in to adventofcode.com

set -e

cd "$(dirname "$(realpath "$0")")/.."

export TZ=US/Eastern

this_year=$(date '+%Y')
today=$(date '+%Y%m%d')

cookies=$(mktemp /tmp/XXXXXXXXXX.sqlite)
find "$HOME/.var/app/org.mozilla.firefox/.mozilla/firefox" -name cookies.sqlite -exec cp {} "$cookies" \;
session=$(sqlite3 "$cookies" "select value from moz_cookies where name = 'session' and host = '.adventofcode.com'")
rm -f "$cookies"

for year in $(seq 2015 "$this_year"); do
    for day in $(seq 25); do
        if [ "$(printf "%4d%02d%02d" "$year" 12 "$day")" -le "$today" ]; then
            fn="input/$year/day$day.txt"
            if [ ! -e "$fn" ]; then
                mkdir -p "input/$year"
                url="https://adventofcode.com/$year/day/$day/input"
                echo "Downloading $url -> $fn"
                curl -sS -b session="$session" "$url" -o $fn
            fi
        fi
    done
done
