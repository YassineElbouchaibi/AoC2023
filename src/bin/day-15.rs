use std::usize;

use advent_of_code_2023::utils::get_aoc_input_lines;

fn main() {
    let mut input_lines = get_aoc_input_lines().expect("Error getting input");
    let input_line = input_lines
        .next()
        .expect("Error reading line")
        .expect("Error reading line");

    // Part 1
    let hashes_sum: usize = input_line.split(",").map(|s| hash(s.trim())).sum();
    println!("Hashes sum: {}", hashes_sum);

    // Part 2
    let mut boxes: Vec<Vec<(String, usize)>> = vec![vec![]; 256];
    let operations = input_line.split(",").map(|s| Operation::from_str(s));
    for operation in operations {
        match operation {
            Operation::Remove(op) => {
                boxes[op.hash].retain(|(label, _)| label != &op.label);
            }
            Operation::Upsert(op) => {
                match boxes[op.hash]
                    .iter_mut()
                    .find(|(label, _)| label == &op.label)
                {
                    Some(existing_box) => {
                        *existing_box = (op.label, op.focal_length);
                    }
                    None => {
                        boxes[op.hash].push((op.label, op.focal_length));
                    }
                }
            }
        }
    }
    let focusing_power = compute_focusing_power(&boxes);
    println!("Focusing power: {}", focusing_power);
}

fn compute_focusing_power(boxes: &Vec<Vec<(String, usize)>>) -> usize {
    boxes.iter().enumerate().map(|(box_idx, the_box)| {
        the_box
            .iter()
            .enumerate()
            .map(|(slot_idx, (_, focal_length))| (box_idx + 1) * (slot_idx + 1) * focal_length)
            .sum::<usize>()
    }).sum()
}

fn hash(input: &str) -> usize {
    input
        .chars()
        .fold(0, |acc, c| 17 * (acc + c as usize) % 256)
}

#[allow(dead_code)]
fn print_boxes(boxes: &Vec<Vec<(String, usize)>>) {
    for (i, box_) in boxes.iter().enumerate() {
        if !box_.is_empty() {
            println!("{}: {:?}", i, box_);
        }
    }
}

enum Operation {
    Remove(OpRemove),
    Upsert(OpUpsert),
}

impl Operation {
    fn from_str(input: &str) -> Operation {
        let input = input.trim();
        if input.ends_with("-") {
            Operation::Remove(OpRemove::from_str(input))
        } else {
            Operation::Upsert(OpUpsert::from_str(input))
        }
    }
}

struct OpRemove {
    label: String,
    hash: usize,
}

impl OpRemove {
    fn from_str(input: &str) -> OpRemove {
        let label = input.trim_end_matches("-");
        let hash = hash(label);
        OpRemove {
            label: label.to_string(),
            hash,
        }
    }
}

struct OpUpsert {
    label: String,
    hash: usize,
    focal_length: usize,
}

impl OpUpsert {
    fn from_str(input: &str) -> OpUpsert {
        let mut parts = input.split("=");
        let label = parts.next().expect("Error reading label").trim();
        let focal_length = parts
            .last()
            .expect("Error reading focal length")
            .trim()
            .parse::<usize>()
            .expect("Error parsing focal length");
        let hash = hash(label);
        OpUpsert {
            label: label.to_string(),
            hash,
            focal_length,
        }
    }
}
