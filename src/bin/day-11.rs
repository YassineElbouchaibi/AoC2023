use std::{io::{BufReader, Lines}, fs::File, collections::{HashSet, HashMap}};

use advent_of_code_2023::utils::get_aoc_input_lines;

fn main() {
    let input_lines = get_aoc_input_lines().expect("Error getting input");
    let space_grid = SpaceGrid::from_input_lines(input_lines);
    
    let expanded_space_grid = space_grid.expand(2);
    let galaxy_distances = expanded_space_grid.get_galaxy_distances();
    println!("Sum of galaxy distances: {}", galaxy_distances.values().sum::<usize>());

    let expanded_space_grid = space_grid.expand(1000000);
    let galaxy_distances = expanded_space_grid.get_galaxy_distances();
    println!("Sum of galaxy distances: {}", galaxy_distances.values().sum::<usize>());
}

#[derive(Clone, Copy)]
enum Space {
    Galaxy,
    Empty,
}

impl Space {
    fn from_char(c: char) -> Space {
        match c {
            '.' => Space::Empty,
            '#' => Space::Galaxy,
            _ => panic!("Invalid space character"),
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        match self {
            Space::Galaxy => print!("#"),
            Space::Empty => print!("."),
        }
    }
}

type RowCoord = usize;
type ColCoord = usize;
type Coords = (RowCoord, ColCoord);
type Weight = usize;

struct SpaceGrid {
    grid: Vec<Vec<(Space, Weight)>>,
}

impl SpaceGrid {
    fn from_input_lines(input_lines: Lines<BufReader<File>>) -> SpaceGrid {
        let mut grid = Vec::new();
        for line in input_lines {
            let line = line.expect("Error reading line");
            let mut row = Vec::new();
            for c in line.chars() {
                row.push((Space::from_char(c), 1));
            }
            grid.push(row);
        }
        SpaceGrid { grid }
    }

    #[allow(dead_code)]
    fn print_grid(&self) {
        for row in &self.grid {
            for space in row {
                match space.0 {
                    Space::Galaxy => print!("#"),
                    Space::Empty => print!("{}", space.1),
                }
            }
            println!();
        }
    }

    fn expand(&self, weigth: usize) -> SpaceGrid {
        let galaxies = self.get_galaxies();
        let galaxy_rows = galaxies.iter().map(|(row, _)| row).collect::<HashSet<_>>();
        let galaxy_cols = galaxies.iter().map(|(_, col)| col).collect::<HashSet<_>>();

        let mut new_grid = Vec::new();
        for (row_coord, row) in self.grid.iter().enumerate() {
            let mut new_row: Vec<(Space, Weight)> = Vec::new();
            
            for (col_coord, space) in row.iter().enumerate() {                
                if !galaxy_cols.contains(&col_coord) && !galaxy_rows.contains(&row_coord) {
                    new_row.push((space.0, 2 * weigth));
                } else if !galaxy_cols.contains(&col_coord) || !galaxy_rows.contains(&row_coord) {
                    new_row.push((space.0, weigth));
                } else {
                    new_row.push(*space);
                }
            }
            
            new_grid.push(new_row);
        }

        SpaceGrid { grid: new_grid }
    }

    fn get_galaxies(&self) -> HashSet<Coords> {
        let mut galaxies = HashSet::new();
        for (row_coord, row) in self.grid.iter().enumerate() {
            for (col_coord, space) in row.iter().enumerate() {
                if let Space::Galaxy = space.0 {
                    galaxies.insert((row_coord, col_coord));
                }
            }
        }
        galaxies
    }

    fn get_galaxy_distances(&self) -> HashMap<(Coords, Coords), usize> {
        let galaxies = self.get_galaxies();
        let mut galaxy_distances = HashMap::new();
        for galaxy_a in &galaxies {
            for galaxy_b in &galaxies {
                if galaxy_a == galaxy_b || galaxy_distances.contains_key(&(*galaxy_b, *galaxy_a)) || galaxy_distances.contains_key(&(*galaxy_a, *galaxy_b)) {
                    continue;
                }

                let distance = self.compute_weigthed_distance(*galaxy_a, *galaxy_b);
                galaxy_distances.insert((*galaxy_a, *galaxy_b), distance);
            }
        }
        galaxy_distances
    }

    fn compute_weigthed_distance(&self, a: Coords, b: Coords) -> usize {
        let (_, a_col) = a;
        let (b_row, _) = b;
    
        let visited_rows = compute_visited_rows(a, b);
        let visited_cols = compute_visited_cols(a, b);
    
        let mut distance = 0;
        for row in visited_rows {
            distance += self.grid[row][a_col].1;
        }
        for col in visited_cols {
            distance += self.grid[b_row][col].1;
        }
    
        distance
    }
}

fn compute_visited_rows(a: Coords, b: Coords) -> HashSet<usize> {
    let (a_row, _) = a;
    let (b_row, _) = b;
    
    if a_row > b_row { 
        (b_row..a_row).collect()
    } else { 
        ((a_row + 1)..=b_row).collect()
    }
}

fn compute_visited_cols(a: Coords, b: Coords) -> HashSet<usize> {
    let (_, a_col) = a;
    let (_, b_col) = b;
    
    if a_col > b_col { 
        (b_col..a_col).collect()
    } else { 
        ((a_col + 1)..=b_col).collect()
    }
}