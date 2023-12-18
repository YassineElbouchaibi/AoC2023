use advent_of_code_2023::utils::get_aoc_input_lines;
use itertools::Itertools;
use pathfinding::prelude::dijkstra;

const MIN_CONSECUCTIVE_DIRECTION: u32 = 4; // Set to 1 for part 1
const MAX_CONSECUCTIVE_DIRECTION: u32 = 10; // Set to 3 for part 1

fn main() {
    let input_lines = get_aoc_input_lines().expect("Error getting input");
    let grid = input_lines
        .map(|line| line.expect("Error reading line"))
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("Failed converting char to digit"))
                .collect_vec()
        })
        .collect_vec();

    let min_cost = dijkstra(
        &Pos::start(0, 0),
        |p| p.succesors(&grid),
        |p| p.row == (grid.len() as i32 - 1) && p.column == (grid[0].len() as i32 - 1),
    );

    println!("Min cost: {:?}", min_cost.unwrap().1);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Start,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Start => panic!("Cannot get opposite of start"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Pos {
    row: i32,
    column: i32,
    direction: Direction,
    consecutive_direction_count: u32,
}

impl Pos {
    fn start(row: i32, column: i32) -> Self {
        Self {
            row,
            column,
            direction: Direction::Start,
            consecutive_direction_count: 1,
        }
    }

    fn go_same_direction(&self) -> Self {
        match self.direction {
            Direction::Up => self.go_up(),
            Direction::Down => self.go_down(),
            Direction::Left => self.go_left(),
            Direction::Right => self.go_right(),
            Direction::Start => panic!("Cannot go same direction from start"),
        }
    }

    fn go_right(&self) -> Self {
        Self {
            row: self.row,
            column: self.column + 1,
            direction: Direction::Right,
            consecutive_direction_count: if self.direction == Direction::Right {
                self.consecutive_direction_count + 1
            } else {
                1
            },
        }
    }

    fn go_left(&self) -> Self {
        Self {
            row: self.row,
            column: self.column - 1,
            direction: Direction::Left,
            consecutive_direction_count: if self.direction == Direction::Left {
                self.consecutive_direction_count + 1
            } else {
                1
            },
        }
    }

    fn go_up(&self) -> Self {
        Self {
            row: self.row - 1,
            column: self.column,
            direction: Direction::Up,
            consecutive_direction_count: if self.direction == Direction::Up {
                self.consecutive_direction_count + 1
            } else {
                1
            },
        }
    }

    fn go_down(&self) -> Self {
        Self {
            row: self.row + 1,
            column: self.column,
            direction: Direction::Down,
            consecutive_direction_count: if self.direction == Direction::Down {
                self.consecutive_direction_count + 1
            } else {
                1
            },
        }
    }

    fn into_successor(self, grid: &Vec<Vec<u32>>) -> (Self, u32) {
        let cost = grid[self.row as usize][self.column as usize];
        (self, cost)
    }

    fn is_out_of_bounds(&self, grid: &Vec<Vec<u32>>) -> bool {
        self.row < 0
            || self.row >= grid.len() as i32
            || self.column < 0
            || self.column >= grid[0].len() as i32
    }

    fn exceeds_consecutive_direction(&self) -> bool {
        self.consecutive_direction_count > MAX_CONSECUCTIVE_DIRECTION
    }

    fn succesors(&self, grid: &Vec<Vec<u32>>) -> Vec<(Self, u32)> {
        if self.direction == Direction::Start {
            return vec![
                self.go_right().into_successor(grid),
                self.go_down().into_successor(grid),
            ];
        }

        if self.consecutive_direction_count < MIN_CONSECUCTIVE_DIRECTION {
            let new_pos = self.go_same_direction();
            return if new_pos.is_out_of_bounds(grid) {
                vec![]
            } else {
                vec![new_pos.into_successor(grid)]
            };
        }

        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .iter()
        .filter(|direction| **direction != self.direction.opposite())
        .map(|direction| {
            let new_pos = match direction {
                Direction::Up => self.go_up(),
                Direction::Down => self.go_down(),
                Direction::Left => self.go_left(),
                Direction::Right => self.go_right(),
                Direction::Start => panic!("Cannot go same direction from start"),
            };

            if new_pos.is_out_of_bounds(grid) || new_pos.exceeds_consecutive_direction() {
                return None;
            }

            Some(new_pos.into_successor(grid))
        })
        .filter(|succ| succ.is_some())
        .map(|succ| succ.unwrap())
        .collect_vec()
    }
}

#[allow(dead_code)]
fn print_grid<T>(grid: &Vec<Vec<T>>)
where
    T: std::fmt::Display,
{
    for row in grid.iter() {
        for cell in row.iter() {
            print!("{} ", cell);
        }
        println!();
    }
}
