use std::collections::HashSet;

use advent_of_code_2023::utils::get_aoc_input_lines;
use itertools::Itertools;

fn main() {
    let input_lines = get_aoc_input_lines().expect("Error getting input");
    let grid = input_lines
        .map(|line| {
            let line = line.expect("Error reading line");
            line.chars().map(Object::from_char).collect_vec()
        })
        .collect_vec();

    let style = indicatif::ProgressStyle::with_template(
        "[{elapsed_precise}/{eta_precise}] {bar:40.cyan/blue} {human_pos}/{human_len} ({percent} %) @{per_sec} {msg}"
    ).unwrap();
    let progress_bar = indicatif::ProgressBar::new((grid.len() * 2 + grid[0].len() * 2 - 2) as u64).with_style(style);

    let mut max_energized_tiles_count = 0;
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, _) in row.iter().enumerate() {
            if col_idx != 0 && row_idx != 0 && col_idx != row.len() - 1 && row_idx != grid.len() - 1 {
                continue;
            }

            let initial_direction;
            if col_idx == 0 {
                initial_direction = Coord::new(0, 1);
            } else if col_idx == row.len() - 1 {
                initial_direction = Coord::new(0, -1);
            } else if row_idx == 0 {
                initial_direction = Coord::new(1, 0);
            } else if row_idx == grid.len() - 1 {
                initial_direction = Coord::new(-1, 0);
            } else {
                panic!("Invalid initial direction");
            }

            let mut previous_energized_tiles_count = 0;
            let mut matches_count = 0;
            let mut energized_tiles: HashSet<(Coord, Coord)> = HashSet::new();
            let mut beams = vec![Beam::new(Coord::new(row_idx as i32, col_idx as i32), initial_direction)];
            loop {
                let mut new_beams = vec![];

                for beam in beams.iter_mut() {
                    let object = grid
                        .get(beam.head.row as usize)
                        .map(|row| row.get(beam.head.column as usize))
                        .flatten();
                    if object.is_none() {
                        continue;
                    }

                    energized_tiles.insert((beam.head, beam.direction));

                    if let Some(new_beam) = beam.process_encounter(object.unwrap()) {
                        if !energized_tiles.contains(&(new_beam.head, new_beam.direction)) {
                            new_beams.push(new_beam);
                        }
                    }
                }

                for new_beam in new_beams {
                    beams.push(new_beam);
                }

                if beams.iter().all(|beam| beam.head.is_out_of_bounds(&grid)) {
                    break;
                }

                let energized_tiles_count = energized_tiles.iter().map(|(coord, _)| coord).unique().count();
                if previous_energized_tiles_count == energized_tiles_count {
                    matches_count += 1;
                    if matches_count == 10 {
                        break;
                    }
                } else {
                    matches_count = 0;
                }
                previous_energized_tiles_count = energized_tiles_count;
            }


            let energized_tiles_count = energized_tiles.iter().map(|(coord, _)| coord).unique().count();
            if energized_tiles_count > max_energized_tiles_count {
                max_energized_tiles_count = energized_tiles_count;
            }

            progress_bar.inc(1);
        }
    }
    progress_bar.finish();
    println!("Max Energized Tiles Count: {}", max_energized_tiles_count);
}

#[derive(Debug, Clone)]
enum ObjectKind {
    Empty,
    Mirror(Mirror),
    Splitter(Splitter),
}

#[derive(Debug, Clone)]
struct Mirror {
    factor: i32,
}

#[derive(Debug, Clone)]
struct Splitter {
    direction: Coord,
}

#[derive(Debug, Clone)]
struct Object {
    kind: ObjectKind,
}

impl Object {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self {
                kind: ObjectKind::Empty,
            },
            '/' => Self {
                kind: ObjectKind::Mirror(Mirror { factor: -1 }),
            },
            '\\' => Self {
                kind: ObjectKind::Mirror(Mirror { factor: 1 }),
            },
            '|' => Self {
                kind: ObjectKind::Splitter(Splitter {
                    direction: Coord::new(1, 0),
                }),
            },
            '-' => Self {
                kind: ObjectKind::Splitter(Splitter {
                    direction: Coord::new(0, 1),
                }),
            },
            _ => panic!("Invalid character"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coord {
    row: i32,
    column: i32,
}

impl Coord {
    fn new(row: i32, column: i32) -> Self {
        Self { row, column }
    }

    fn swap(&mut self) {
        let temp = self.row;
        self.row = self.column;
        self.column = temp;
    }

    fn multiply(&mut self, factor: i32) {
        self.row *= factor;
        self.column *= factor;
    }

    fn is_out_of_bounds(&self, grid: &Vec<Vec<Object>>) -> bool {
        self.row < 0
            || self.column < 0
            || self.row >= grid.len() as i32
            || self.column >= grid[0].len() as i32
    }
}

#[derive(Debug, Clone)]
struct Beam {
    head: Coord,
    direction: Coord,
}

impl Beam {
    fn new(head: Coord, direction: Coord) -> Self {
        Self {
            head,
            direction,
        }
    }

    fn step(&mut self) {
        self.head.row += self.direction.row;
        self.head.column += self.direction.column;
    }

    fn process_encounter(&mut self, object: &Object) -> Option<Beam> {
        // Depending on the object, step in its direction
        // and change the beam's direction
        // May return a new beam if the object is a splitter
        match &object.kind {
            ObjectKind::Empty => {
                self.step();
            }
            ObjectKind::Mirror(mirror) => {
                self.direction.swap();
                self.direction.multiply(mirror.factor);
                self.step();
            }
            ObjectKind::Splitter(splitter) => {
                // println!("Encountered splitter: {:?} at {:?}. Current Dir: {:?}", splitter, self.head, self.direction);
                if self.direction.row == 0 && splitter.direction.row != 0
                || self.direction.column == 0 && splitter.direction.column != 0
                {
                    self.direction.swap();
                    
                    let mut new_beam = self.clone();
                    new_beam.direction.multiply(-1);

                    self.step();
                    new_beam.step();
                    
                    return Some(new_beam);
                } else {
                    self.step();
                }
            }
        }

        None
    }
}

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<Object>>, energized_tiles: &Vec<&Coord>) {
    for (row_index, row) in grid.iter().enumerate() {
        for (column_index, object) in row.iter().enumerate() {
            let coord = Coord::new(row_index as i32, column_index as i32);
            if energized_tiles.contains(&&coord) {
                print!("#");
            } else {
                match &object.kind {
                    ObjectKind::Empty => print!("."),
                    ObjectKind::Mirror(mirror) => {
                        if mirror.factor == 1 {
                            print!("\\");
                        } else {
                            print!("/");
                        }
                    }
                    ObjectKind::Splitter(splitter) => {
                        if splitter.direction.row == 1 {
                            print!("|");
                        } else {
                            print!("-");
                        }
                    }
                }
            }
        }
        println!();
    }
}