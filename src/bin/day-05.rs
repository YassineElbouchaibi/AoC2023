use std::{io::{Lines, BufReader}, fs::File, collections::HashSet};

use advent_of_code_2023::utils::get_aoc_input_lines;
use itertools::min;

fn main() {
    let mut input_lines = get_aoc_input_lines().expect("Error getting input");
    let mut seeds: Vec<(i64, i64)> = Vec::new();

    while let Some(line) = input_lines.next() {
        let line = line.expect("Error getting line");

        if line.starts_with("seeds:") {
            seeds = process_seeds(&line);
            continue;
        }

        if line.ends_with("map:") {
            seeds = process_map(seeds.clone(), &mut input_lines);
        }
    }

    println!("seeds: {:?}", min(seeds).unwrap().0);
}

fn process_seeds(line: &str) -> Vec<(i64, i64)> {
    let line = line.split(":").last().expect("Error splitting seeds");
    let line = line.trim();

    line.split_whitespace().map(|seed| {
        seed.parse::<i64>().expect("Error parsing seed")
    }).collect::<Vec<_>>().chunks(2).map(|pair| {
        (pair[0], pair[1])
    }).collect::<Vec<_>>()
}

fn process_map(seeds: Vec<(i64, i64)>, input_lines: &mut Lines<BufReader<File>>) -> Vec<(i64, i64)> {
    let mut confirmed_seeds = HashSet::new();
    let mut seeds = seeds;
    let map_lines = input_lines.take_while(|line| {
        let line = line.as_ref().expect("Error getting line while processing map");
        !line.trim().is_empty()
    }).collect::<Vec<_>>();

    let mut previous_seeds = seeds.clone();
    loop {
        for line in map_lines.iter() {
            let line = line.as_ref().expect("Error getting line while processing map");
    
            let (destination, source, length) = process_map_line(&line);
            seeds = seeds.iter().flat_map(|seed| {
                let seed = *seed;
    
                if seed.0 >= source && seed.0 < source + length && !confirmed_seeds.contains(&seed) {
                    let new_seed_source = seed.0 + destination - source;
                    let covered_count = length - (seed.0 - source);
                    if covered_count >= seed.1 {
                        confirmed_seeds.insert((new_seed_source, seed.1));
                        return vec![(new_seed_source, seed.1)];
                    } else {
                        confirmed_seeds.insert((new_seed_source, covered_count));
                        return vec![(new_seed_source, covered_count), (seed.0 + covered_count, seed.1 - covered_count)];
                    }
                } else if source > seed.0 && source < seed.0 + seed.1 && !confirmed_seeds.contains(&seed) {
                    let available_length = seed.0 + seed.1 - source;
                    let map_length = length;
                    let covered_count = std::cmp::min(available_length, map_length);
                    if available_length <= map_length {
                        confirmed_seeds.insert((destination, covered_count));
                        return vec![(seed.0, seed.1 - covered_count), (destination, covered_count)];
                    } else if available_length > map_length {
                        confirmed_seeds.insert((destination, covered_count));
                        return vec![(seed.0, seed.1 - covered_count), (destination, covered_count), (source + covered_count, available_length - map_length)];
                    }
                }
    
                vec![seed]
            }).collect();
        }

        if previous_seeds == seeds {
            break;
        }
        previous_seeds = seeds.clone();
    }

    seeds
}

fn process_map_line(line: &str) -> (i64, i64, i64) {
    let mut line = line.split_whitespace();
    let destination = line.next().expect("Error getting destination").parse::<i64>().expect("Error parsing destination");
    let source = line.next().expect("Error getting source").parse::<i64>().expect("Error parsing source");
    let length = line.next().expect("Error getting length").parse::<i64>().expect("Error parsing length");

    (destination, source, length)
}