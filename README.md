# Advent of Code 2023

This repository contains my solutions for the Advent of Code 2023. It marks my first serious foray into Rust programming, with my initial exposure to the language dating back over a year ago. However, that initial interaction was brief and exploratory, lasting only a couple of days. The primary objective of this project was to serve as a Rust training ground, allowing me to gradually build my skills in the language. I embarked on this journey with the hope that the code quality would improve day by day as I delved deeper into Rust development. This repository stands as a testament to my learning progress and commitment to mastering Rust over time.

## Scaffolding a New AoC Day

To scaffold the code needed for a new AoC day, use the `new-day.sh` script. Run it in your terminal as follows:

```bash
./new-day.sh <DAY>
```
Replace `<DAY>` is the day number. e.g. `./new-day.sh 01`

## Fetching Input for a Given Day

To fetch the input for a given day, use the `fetch-input.sh` script. Run it in your terminal as follows:

```bash
./fetch-input.sh <DAY>
```
Replace `<DAY>` with the day number. e.g. `./fetch-input.sh 01`

## Building the Binaries

To build the binaries, use the `cargo build` command. For a debug build, run:

```bash
cargo build
```

For a release build, run:

```bash
cargo build --release
```

## Executing Binaries

To execute the binaries in debug mode, use the `cargo run` command:

```bash
cargo run --bin day-<DAY> <INPUT_FILE>
```

To execute them in release mode, use:

```bash
cargo run --release --bin day-<DAY> <INPUT_FILE>
```

Replace `<DAY>` with the day number and `<INPUT_FILE>` with the path to the input file. e.g. `cargo run --bin day-01 inputs/day-01.txt`

## Executing Binaries Without Cargo

To execute the binaries without using cargo, navigate to the `target/debug` or `target/release` directory and run:

```bash
./day-<DAY> <INPUT_FILE>
```

Replace `<DAY>` with the day number and `<INPUT_FILE>` with the path to the input file. e.g. `target/release/day-01 inputs/day-01.txt`

## Input File Argument

All binaries take a single argument that is the path to the input file. For example:

```bash
target/release/day-01 inputs/day-01.txt
```

My input files can be found in the `inputs` directory and sample input files can be found in the `sample-inputs` directory.

## Output

Depending on personal life time constraints and how the problem was approached, every binary either outputs the answer to both parts of the AoC question or only the second part.