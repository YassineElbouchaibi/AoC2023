use std::{str::Chars, collections::HashSet};
use lazy_static::lazy_static;
use advent_of_code_2023::utils::get_aoc_input_lines;
use either::Either;
use phf::phf_map;

const SPELLED_DIGITS_MIN_LENGTH: usize = 3;
const SPELLED_DIGITS_MAX_LENGTH: usize = 5;
static SPELLED_DIGITS_MAP: phf::Map<&'static str, usize> = phf_map! {
    "one" => 1,
    "two" => 2,
    "three" => 3,
    "four" => 4,
    "five" => 5,
    "six" => 6,
    "seven" => 7,
    "eight" => 8,
    "nine" => 9,
};

lazy_static!  {
    #[derive(Debug)]
    static ref SPELLED_DIGITS_PREFIXES_SET: HashSet<&'static str> = SPELLED_DIGITS_MAP.keys().flat_map(|key| {
        let mut keys = Vec::with_capacity(key.len());

        // Add all prefixes of the key (including the key itself)
        for i in 1..key.len() + 1 {
            keys.push(key.get(0..i).unwrap());
        }
        
        keys
    }).collect();

    #[derive(Debug)]
    static ref SPELLED_DIGITS_SUFFIXES_SET: HashSet<&'static str> = SPELLED_DIGITS_MAP.keys().flat_map(|key| {
        let mut keys = Vec::with_capacity(key.len());

        // Add all suffixes of the key
        for i in 0..key.len() + 1 {
            keys.push(key.get(i..key.len()).unwrap());
        }
        
        keys
    }).collect();
}

fn main() {
    let input_lines = get_aoc_input_lines().expect("Error getting input");

    let mut sum = 0;

    for line in input_lines {
        let line = line.expect("Error reading line");
        let mut chars = line.chars();
        
        let first_digit = match get_digit(&mut chars, false) {
            Some(digit) => digit,
            None => continue,
        };
        
        let second_digit = match get_digit(&mut chars, true) {
            Some(digit) => digit,
            None => first_digit,
        };

        sum += 10 * first_digit + second_digit;
    }

    println!("{}", sum);
}

fn get_digit(chars: &mut Chars<'_>, rev: bool) -> Option<u32> {
    let mut spelled_digit = String::with_capacity(SPELLED_DIGITS_MAX_LENGTH);
    
    let chars = if rev {
        Either::Left(chars.rev())
    } else {
        Either::Right(chars)
    };

    for char in chars {
        // Try to parse the character as a digit.
        if char.is_digit(10) {
            return char.to_digit(10);
        }

        // The character is not a digit, so add it to the spelled digit.
        if rev {
            spelled_digit.insert(0, char);
        } else {
            spelled_digit.push(char);
        }

        // If spelled_digit is a not prefix of a spelled digit, continue.
        if rev {
            while !SPELLED_DIGITS_SUFFIXES_SET.contains(spelled_digit.as_str()) && spelled_digit.len() != 0 {
                spelled_digit.pop();
            }
        } else {
            while !SPELLED_DIGITS_PREFIXES_SET.contains(spelled_digit.as_str()) && spelled_digit.len() != 0 {
                spelled_digit.remove(0);
            }
        }

        // If the spelled digit is too long, return None.
        if spelled_digit.len() > SPELLED_DIGITS_MAX_LENGTH || spelled_digit.len() == 0 {
            continue;
        }

        // If the spelled digit is long enough, try to parse it.
        if spelled_digit.len() >= SPELLED_DIGITS_MIN_LENGTH {
            if let Some(digit) = SPELLED_DIGITS_MAP.get(spelled_digit.as_str()) {
                return Some(*digit as u32);
            }
        }
    }

    None
}