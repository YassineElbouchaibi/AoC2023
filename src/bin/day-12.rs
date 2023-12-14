use std::fmt::Display;

use cached::proc_macro::cached;
use itertools::{repeat_n, Itertools};

use advent_of_code_2023::utils::get_aoc_input_lines;

fn main() {
    let input_lines = get_aoc_input_lines().expect("Error getting input");

    let mut permutations_count = 0;
    for line in input_lines {
        let line = line.expect("Error reading line");

        let condition_record = ConditionRecord::from_string(&line).repeat(5);
        permutations_count += condition_record.count_permutations();
    }

    println!("Permutations count: {}", permutations_count);
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
enum Spring {
    Operationnal,
    Damaged,
    Unknown,
}

impl Spring {
    fn from_char(c: char) -> Spring {
        match c {
            '.' => Spring::Operationnal,
            '#' => Spring::Damaged,
            '?' => Spring::Unknown,
            _ => panic!("Invalid char for Spring"),
        }
    }
}

impl Display for Spring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Spring::Operationnal => '.',
            Spring::Damaged => '#',
            Spring::Unknown => '?',
        };

        write!(f, "{}", c)
    }
}

#[derive(Debug, Clone)]
struct ConditionRecord {
    springs: Vec<Spring>,
    damaged_sequence: Vec<i64>,
}

impl ConditionRecord {
    fn from_string(s: &str) -> ConditionRecord {
        let mut split_string = s.split_whitespace();

        let springs = split_string.next().expect("Error reading springs");
        let springs = springs.chars().map(|c| Spring::from_char(c)).collect();

        let damaged_sequence = split_string.next().expect("Error reading damaged sequence");
        let damaged_sequence = damaged_sequence
            .split(',')
            .map(|s| s.parse::<i64>().expect("Error parsing damaged sequence"))
            .collect();

        ConditionRecord {
            springs,
            damaged_sequence,
        }
    }

    fn repeat(&self, n: usize) -> ConditionRecord {
        if n == 0 {
            return ConditionRecord {
                springs: vec![],
                damaged_sequence: vec![],
            }
        }

        if n == 1 {
            return self.clone();
        }

        let springs = self.springs.clone();

        ConditionRecord {
            springs: repeat_n(springs, n).enumerate().map(|(i, mut s)| {
                if (i + 1) != n {
                    s.push(Spring::Unknown);
                }
                s
            }).flatten().collect_vec(),
            damaged_sequence: repeat_n(self.damaged_sequence.clone(), n)
                .flatten()
                .collect_vec(),
        }
    }

    fn count_permutations(&self) -> i64 {
        count_permutations(self.springs.clone(), self.damaged_sequence.clone(), true)
    }
}

#[cached(
    key = "String",
    convert = r#"{ format!("{:?}-{:?}-{}", springs, damaged_sequence, is_initial_call) }"#
)]
fn count_permutations(springs: Vec<Spring>, damaged_sequence: Vec<i64>, is_initial_call: bool) -> i64 {
    if damaged_sequence.len() == 0 {
        return if springs.contains(&Spring::Damaged) {0} else {1};
    }

    let size = damaged_sequence[0];
    let new_damaged_sequence = damaged_sequence[1..].to_vec();

    let mut permutations_count = 0;
    for end in 0..springs.len() {
        let start = end as i64 - size + 1;

        if valid_permutation(&springs, start, end as i64, is_initial_call) {
            permutations_count += count_permutations(springs[end + 1..].to_vec(), new_damaged_sequence.clone(), false);
        }
    }

    permutations_count
}

fn valid_permutation(springs: &Vec<Spring>, start: i64, end: i64, is_initial_call: bool) -> bool {
    // Not out of bounds
    if start < 0 || end >= springs.len() as i64 {
        return false;
    }

    // On subsequent calls, there might be a damaged part on the outside edges
    // Therefore the edges cannot be used
    if !is_initial_call && start == 0 {
        return false;
    }

    let start = start as usize;
    let end = end as usize;

    if  // We have spacing around the damaged sequence
        springs.get(start - 1) == Some(&Spring::Damaged)
        || springs.get(end + 1) == Some(&Spring::Damaged)
        // There cannot be a damaged part before the start
        || springs[0..start].contains(&Spring::Damaged)
        // Start to end cannot be operationnal
        || springs[start..=end].contains(&Spring::Operationnal)
    {
        return false;
    }

    true
}
