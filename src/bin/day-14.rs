use advent_of_code_2023::utils::get_aoc_input_lines;
use indicatif::ProgressIterator;
use itertools::Itertools;

fn main() {
    let input_lines = get_aoc_input_lines().expect("Error getting input");
    let grid = input_lines
        .map(|line| line.expect("Error reading line").chars().collect_vec())
        .collect_vec();

    let tilted_grid = tilt_grid_n_cycles(&grid, 1_000); // Cycle repeats, the 1_000th cycle is the same as the 1_000_000_000th
    let total_load = compute_grid_load(&tilted_grid);

    println!("Total load: {}", total_load);
}

fn compute_grid_load(grid: &Vec<Vec<char>>) -> usize {
    let row_count = grid.len();
    grid.iter()
        .enumerate()
        .map(|(i, row)| (row_count - i) * row.iter().filter(|cell| **cell == 'O').count())
        .sum()
}

fn tilt_grid_n_cycles(grid: &Vec<Vec<char>>, n: usize) -> Vec<Vec<char>> {
    let mut tilted_grid = grid.clone();
    let style = indicatif::ProgressStyle::with_template(
        "[{elapsed_precise}/{eta_precise}] {bar:40.cyan/blue} {human_pos:>7}/{human_len:7} ({percent} %) @{per_sec} {msg}"
    ).unwrap();
    for _ in (0..n).progress_with_style(style) {
        tilted_grid = tilt_grid_up(&tilted_grid);
        tilted_grid = tilt_grid_left(&tilted_grid);
        tilted_grid = tilt_grid_down(&tilted_grid);
        tilted_grid = tilt_grid_right(&tilted_grid);
    }
    tilted_grid
}

fn tilt_grid_up(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let grid = transpose(grid);
    let grid = grid
        .iter()
        .map(|column| tilt_row_left(column))
        .collect_vec();
    transpose(&grid)
}

fn tilt_grid_down(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let grid = transpose(grid);
    let grid = grid
        .iter()
        .map(|column| {
            tilt_row_left(&column.iter().rev().cloned().collect_vec())
                .iter()
                .rev()
                .cloned()
                .collect_vec()
        })
        .collect_vec();
    transpose(&grid)
}

fn tilt_grid_left(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    grid.iter().map(|row| tilt_row_left(row)).collect_vec()
}

fn tilt_grid_right(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    grid.iter()
        .map(|row| {
            tilt_row_left(&row.iter().rev().cloned().collect_vec())
                .iter()
                .rev()
                .cloned()
                .collect_vec()
        })
        .collect_vec()
}

fn transpose(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let width = grid.get(0).map(|row| row.len()).unwrap_or(0);
    (0..width)
        .map(|i| grid.iter().map(|row| row[i]).collect_vec())
        .collect_vec()
}

fn tilt_row_left(column: &Vec<char>) -> Vec<char> {
    let mut tilted_column = Vec::new();
    let mut insertion_index = 0;
    for (i, cell) in column.iter().enumerate() {
        match *cell {
            '.' => {
                tilted_column.push(*cell);
            }
            '#' => {
                tilted_column.push(*cell);
                insertion_index = i + 1;
            }
            'O' => {
                tilted_column.insert(insertion_index, *cell);
            }
            _ => panic!("Invalid cell value: {}", cell),
        }
    }

    tilted_column
}

#[allow(dead_code)]
fn format_row(row: &Vec<char>) -> String {
    row.iter().collect()
}

fn format_grid(grid: &Vec<Vec<char>>) -> String {
    let mut representation = String::new();
    for row in grid {
        for cell in row {
            representation.push(*cell);
        }
        representation.push('\n');
    }
    representation
}

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<char>>) {
    print!("{}", format_grid(grid));
}
