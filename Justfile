default:
  just --list

# Create a code template for year/day
setup year day:
  ./bin/setup.sh {{year}} {{day}}

# Create a code template for today's puzzle
setup-today:
  ./bin/setup

# Run a year/day
run year day:
  cargo run {{year}} {{day}}

# Run a year/day (optimised)
run-fast year day:
  cargo run --release {{year}} {{day}}

# Run an entire year
run-year year:
  cargo run --release {{year}}

# Run tests for a single year/day
test year day:
  cargo test {{year}}::day{{day}} -- --nocapture
