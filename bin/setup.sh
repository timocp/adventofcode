#! /bin/sh

set -e

mkdir -p src

year=$1
day=$2

if [ "$year" = "" ]; then
    year=$(TZ=US/Eastern date '+%Y')
fi

if [ "$day" = "" ]; then
    day=$(TZ=US/Eastern date '+%d' | sed 's/^0//')
fi

year_rs="src/y$year.rs"
year_dir="src/y$year"
day_rs="src/y$year/day$day.rs"

if ! git diff --exit-code > /dev/null; then
    echo "There are uncommitted changes" 2>&1
    exit 1
fi
if [ -e "$day_rs" ]; then
    echo "Already exists: $day_rs" 2>&1
    exit 1
fi

./bin/download.sh

echo "Creating $day_rs..."
mkdir -p "$year_dir"
cat > "$day_rs" <<EOF
pub fn parse_input(input: &str) -> Vec<u32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn part1(input: &[i32]) -> &str {
    "unimplemented"
}

pub fn part2(input: &[i32]) -> &str {
    "unimplemented"
}

#[test]
fn test() {
    let test_input = "\
";
    // assert_eq!()
}
EOF

echo "Updating $year_rs..."
echo "pub mod day$day;" >> $year_rs

# TODO: insert into main.rs (tricky)

cargo fmt
