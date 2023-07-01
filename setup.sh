#! /bin/sh

set -e

mkdir -p src

day=$1

if [ "$day" = "" ]; then
    day=$(TZ=US/Eastern date '+%d' | sed 's/^0//')
fi

src="src/day$day.rs"

if ! git diff --exit-code > /dev/null; then
    echo "There are uncommitted changes" 2>&1
    exit 1
fi
if [ -e "$src" ]; then
    echo "Already exists: $src" 2>&1
    exit 1
fi

../aoc-input/download.sh

echo "Creating $src..."
cat > "$src" <<EOF
use crate::Part;

pub fn run(input: &str, part: Part) -> String {
    // let input = parse_input(input);
    format!(
        "{}",
        match part {
            Part::One => "?",
            Part::Two => "?",
        }
    )
}

#[test]
fn test() {
    let test_input = "\
";
    // assert_eq!()
}
EOF
