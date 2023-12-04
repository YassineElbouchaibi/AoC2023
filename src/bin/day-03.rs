use std::{
    cmp::max,
    collections::{HashMap, HashSet},
    iter::Peekable,
    str::Chars,
};

use advent_of_code_2023::utils::get_aoc_input_lines;

fn main() {
    let input_lines = get_aoc_input_lines().expect("Error getting input");
    let mut sum_part_numbers = 0;
    let mut sum_gear_ratios = 0;

    let mut gears = HashMap::<(i32, i32), Symbol>::new();

    let mut previous_line_symbol_idxs = HashSet::<i32>::new();
    let mut previous_line_part_numbers = Vec::<PartNumber>::new();

    let mut current_line_symbol_idxs = Vec::<i32>::new();
    let mut current_line_part_numbers = Vec::<PartNumber>::new();

    let mut line_idx = 0;
    for line in input_lines {
        let line = line.expect("Error reading line");

        let mut chars = line.chars().peekable();
        let mut idx = 0;

        loop {
            if chars.peek().is_none() {
                break;
            }

            let symbol = next_symbol_idx(&mut chars, idx);
            if symbol.is_some() {
                let symbol = symbol.unwrap();
                idx = symbol.idx + 1; // Symbol index is inclusive, so add 1

                if symbol.symbol_type == SymbolType::Gear {
                    gears.insert((-line_idx, symbol.idx), symbol.clone());
                }

                if symbol.symbol_type == SymbolType::Other || symbol.symbol_type == SymbolType::Gear
                {
                    current_line_symbol_idxs.push(symbol.idx);

                    let mut validated_idxs = HashSet::<i32>::new();
                    for part_number in previous_line_part_numbers.iter() {
                        if symbol.idx >= (part_number.range.0 - 1)
                            && symbol.idx <= part_number.range.1
                        {
                            match gears.get_mut(&(-line_idx, symbol.idx)) {
                                Some(gear) => gear.neighbours.push(part_number.clone()),
                                None => (),
                            }
                            sum_part_numbers += part_number.number;
                            validated_idxs.insert(part_number.range.0);
                        }
                    }
                    previous_line_part_numbers
                        .retain(|part_number| !validated_idxs.contains(&part_number.range.0));
                    validated_idxs.clear();

                    for part_number in current_line_part_numbers.iter() {
                        if symbol.idx >= (part_number.range.0 - 1)
                            && symbol.idx <= part_number.range.1
                        {
                            match gears.get_mut(&(-line_idx, symbol.idx)) {
                                Some(gear) => gear.neighbours.push(part_number.clone()),
                                None => (),
                            }
                            sum_part_numbers += part_number.number;
                            validated_idxs.insert(part_number.range.0);
                        }
                    }
                    current_line_part_numbers
                        .retain(|part_number| !validated_idxs.contains(&part_number.range.0));
                    validated_idxs.clear();
                }

                continue;
            }

            let part_number = PartNumber::from_input_chars(&mut chars, idx);
            if part_number.is_some() {
                let part_number = part_number.unwrap();
                idx = part_number.range.1; // Part number range is exclusive, so use the end index

                let mut pending_validation = true;
                for i in max(0, part_number.range.0 - 1)..(part_number.range.1 + 1) {
                    if previous_line_symbol_idxs.contains(&i)
                        || current_line_symbol_idxs.contains(&i)
                    {
                        if previous_line_symbol_idxs.contains(&i) {
                            match gears.get_mut(&(-line_idx + 1, i)) {
                                Some(gear) => gear.neighbours.push(part_number.clone()),
                                None => (),
                            }
                        } else {
                            match gears.get_mut(&(-line_idx, i)) {
                                Some(gear) => gear.neighbours.push(part_number.clone()),
                                None => (),
                            }
                        }

                        sum_part_numbers += part_number.number;
                        pending_validation = false;
                        break;
                    }
                }

                if pending_validation {
                    current_line_part_numbers.push(part_number);
                }

                continue;
            }
        }

        previous_line_symbol_idxs = current_line_symbol_idxs.drain(..).collect();
        previous_line_part_numbers = current_line_part_numbers.drain(..).collect();
        line_idx += 1;
    }

    for (k, gear) in gears.iter() {
        println!("{:?} - {:?}", k, gear);
        if gear.neighbours.len() == 2 {
            let gear_ratio = gear.neighbours[0].number * gear.neighbours[1].number;
            sum_gear_ratios += gear_ratio;
        }
    }

    println!("Part A: {}", sum_part_numbers);
    println!("Part B: {}", sum_gear_ratios);
}

#[derive(Clone, Copy, Debug)]
struct PartNumber {
    number: i32,
    range: (i32, i32),
}

impl PartNumber {
    fn from_input_chars(input: &mut Peekable<Chars<'_>>, idx_offset: i32) -> Option<PartNumber> {
        //  Exit early if first char is not a digit
        if !input.peek().unwrap().is_digit(10) {
            return None;
        }

        let mut number = 0;
        let mut width = 0;
        loop {
            match input.next_if(|c| c.is_digit(10)) {
                Some(digit) => {
                    number = 10 * number + digit.to_digit(10).unwrap();
                    width += 1;
                }
                None => break,
            }
        }

        Some(PartNumber {
            number: number as i32,
            range: (idx_offset, idx_offset + width),
        })
    }
}

#[derive(Clone, Debug)]
struct Symbol {
    idx: i32,
    symbol_type: SymbolType,
    neighbours: Vec<PartNumber>,
}

#[derive(Clone, PartialEq, Debug)]
enum SymbolType {
    Gear,
    Void,
    Other,
}

fn next_symbol_idx(input: &mut Peekable<Chars<'_>>, idx_offset: i32) -> Option<Symbol> {
    // Exit early if first char is a digit
    if input.peek().unwrap().is_digit(10) {
        return None;
    }

    let mut idx = 0;
    let mut symbol_type = SymbolType::Other;
    loop {
        match input.next_if(|c| !c.is_digit(10)) {
            Some(c) => {
                idx += 1;
                match c {
                    '*' => {
                        symbol_type = SymbolType::Gear;
                        break;
                    }
                    '.' => symbol_type = SymbolType::Void,
                    _ => {
                        symbol_type = SymbolType::Other;
                        break;
                    }
                }
            }
            None => break,
        }
    }

    Some(Symbol {
        idx: idx_offset + idx - 1,
        symbol_type,
        neighbours: Vec::<PartNumber>::new(),
    })
}
