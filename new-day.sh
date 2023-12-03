#!/bin/zsh

set -euo pipefail
SCRIPT_DIR=$(realpath "$(dirname "$0")")

if [[ $# != 1 ]]; then
  echo "Please provide a day number."
  echo "usage: $0 DAY"
  exit 1
fi

if [[ ! "$1" =~ ^(0[1-9]|1[0-9]|2[0-5])$ ]]; then
  echo "Argument '$1' is not a valid day."
  exit 1
fi

# Create src/bin/day-XX.rs with hello world
echo "use advent_of_code_2023::utils::get_aoc_input_lines;

fn main() {
    let input_lines = get_aoc_input_lines().expect(\"Error getting input\");

    for line in input_lines {
        println!(\"{}\", line.expect(\"Error reading line\"));
    }
}" > "$SCRIPT_DIR/src/bin/day-$1.rs"


# Create sample-inputs/day-XX.txt
echo "Sample input for day $1" > "$SCRIPT_DIR/sample-inputs/day-$1.txt"