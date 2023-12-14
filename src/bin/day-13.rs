use std::collections::HashSet;

use advent_of_code_2023::utils::get_aoc_input_lines;
use itertools::Itertools;

fn main() {
    let input_lines = get_aoc_input_lines().expect("Error getting input");

    let mut reflection_summary = 0;
    let mut current_pattern = Vec::<Vec<char>>::new();
    for line in input_lines {
        let line = line.expect("Error reading line");

        if line.trim().is_empty() {
            let (reflection_row, reflection_column) = process_pattern(&current_pattern);
            update_reflection_summary(&mut reflection_summary, reflection_row, reflection_column);
            current_pattern.clear();
        } else {
            current_pattern.push(line.chars().collect_vec());
        }
    }

    let (reflection_row, reflection_column) = process_pattern(&current_pattern);
    update_reflection_summary(&mut reflection_summary, reflection_row, reflection_column);

    println!("Reflection summary: {}", reflection_summary);
}

fn process_pattern(pattern: &Vec<Vec<char>>) -> (Option<usize>, Option<usize>) {
    let reflection_row = find_reflection_row(pattern, None);
    let reflection_column = find_reflection_column(pattern, None);
    let ignored_rows = reflection_row.into_iter().collect::<HashSet<usize>>();
    let ignored_columns = reflection_column.into_iter().collect::<HashSet<usize>>();

    for i in 0..pattern.len() {
        for j in 0..pattern[i].len() {
            let mut pattern = pattern.clone();
            pattern[i][j] = match pattern[i][j] {
                '.' => '#',
                '#' => '.',
                _ => panic!("Invalid character"),
            };
            
            let new_reflection_row = find_reflection_row(&pattern, Some(&ignored_rows));
            if reflection_row != new_reflection_row && new_reflection_row.is_some() {
                return (new_reflection_row, None);
            }

            let new_reflection_column = find_reflection_column(&pattern, Some(&ignored_columns));
            if reflection_column != new_reflection_column && new_reflection_column.is_some() {
                return (None, new_reflection_column);
            }
        }
    }

    (reflection_row, reflection_column)
}

fn update_reflection_summary(
    reflection_summary: &mut usize,
    reflection_row: Option<usize>,
    reflection_column: Option<usize>,
) {
    if let Some(reflection_column) = reflection_column {
        *reflection_summary += reflection_column + 1; // 0-indexed to 1-indexed
    }

    if let Some(reflection_row) = reflection_row {
        *reflection_summary += 100 * (reflection_row + 1); // 0-indexed to 1-indexed
    }
}

fn find_reflection_row(pattern: &Vec<Vec<char>>, ignores: Option<&HashSet::<usize>>) -> Option<usize> {
    pattern
        .iter()
        .enumerate()
        .zip(pattern.iter().skip(1))
        .filter_map(
            |((i, row), next_row)| {
                if row == next_row {
                    Some(i)
                } else {
                    None
                }
            },
        )
        .filter(|initial_guess| {
            let initial_guess = *initial_guess;
            if ignores.map(|ignores| ignores.contains(&initial_guess)).unwrap_or(false) {
                return false;
            }

            let top_range = (0..initial_guess).rev();
            let bottom_range = (initial_guess + 2)..pattern.len();
            let range = top_range.zip(bottom_range);
            for (top, bottom) in range {
                if pattern[top] != pattern[bottom] {
                    return false;
                }
            }

            true
        })
        .next()
}

fn find_reflection_column(pattern: &Vec<Vec<char>>, ignores: Option<&HashSet<usize>>) -> Option<usize> {
    let width = pattern[0].len();
    let pattern = (0..width)
        .map(|i| pattern.iter().map(|row| row[i]).collect_vec())
        .collect_vec();
    find_reflection_row(&pattern, ignores)
}
