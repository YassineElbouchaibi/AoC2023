use std::collections::HashSet;

use advent_of_code_2023::utils::get_aoc_input_lines;

fn main() {
    let input_lines = get_aoc_input_lines().expect("Error getting input");
    let mut points = 0;
    let mut card_counter = Vec::with_capacity(100);

    for (idx, line) in input_lines.enumerate() {
        let line = line.expect("Error reading line");
        
        let card = parse_line(&line);
        let intersection = card.expected_numbers.intersection(&card.numbers);
        let matches = intersection.count();

        card_counter.put_mod(idx, |x| x + 1);
        let next_cards_coeff = card_counter[idx];
        for i in 1..matches + 1 {
            card_counter.put_mod(idx + i, |x| x + next_cards_coeff);
        }

        if matches > 0 {
            points += usize::pow(2, (matches - 1) as u32);
        }
    }

    println!("Points: {}", points);
    println!("Scratch cards: {}", card_counter.iter().sum::<usize>());
}

struct ScratchCard {
    expected_numbers: HashSet<usize>,
    numbers: HashSet<usize>,
}

fn parse_line(line: &str) -> ScratchCard {
    let mut parts = line.split(":").last().expect("Error splittng Card id").split("|");
    let expected_numbers = parse_numbers_to_hashset(parts.next().expect("Error parsing expected numbers").trim());
    let numbers = parse_numbers_to_hashset(parts.next().expect("Error parsing my numbers").trim());

    ScratchCard {
        expected_numbers,
        numbers,
    }
}

fn parse_numbers_to_hashset(numbers: &str) -> HashSet<usize> {
    let mut result = HashSet::new();

    for number in numbers.split_whitespace() {
        result.insert(number.parse::<usize>().expect("Error parsing number"));
    }

    result
}

trait VecPut<T> {
    fn put_mod<F: Fn(T) -> T>(&mut self, index: usize, modifer: F);
}

impl<T> VecPut<T> for Vec<T> where T: Clone, T: Default {
    fn put_mod<F: Fn(T) -> T>(&mut self, index: usize, modifier: F) {
        if self.len() <= index {
            self.resize(index + 1, modifier(Default::default()));
        } else {
            self[index] = modifier(self[index].clone());
        }
    }
}