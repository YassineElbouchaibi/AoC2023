use std::{collections::{BinaryHeap, HashSet, VecDeque}, cmp::Reverse};

use advent_of_code_2023::utils::get_aoc_input_lines;


fn main() {
    let input_lines = get_aoc_input_lines().expect("Error getting input");
    let input_lines = input_lines.map(|line| line.expect("Error reading line"));

    let mut grid: Vec<Vec<Tile>> = input_lines
        .map(|s| s.chars().map(Tile::from_char).collect())
        .collect();

    let start = (0..grid.len())
        .find_map(|i| (0..grid[i].len()).find_map(|j| (grid[i][j].start == true).then_some((i, j))))
        .unwrap();

    grid[start.0][start.1].north = start.0 > 0 && grid[start.0 - 1][start.1].south;
    grid[start.0][start.1].south = start.0 < grid.len() - 1 && grid[start.0 + 1][start.1].north;
    grid[start.0][start.1].west = start.1 > 0 && grid[start.0][start.1 - 1].east;
    grid[start.0][start.1].east = start.1 < grid[start.0].len() - 1 && grid[start.0][start.1 + 1].west;

    // Part 1
    let mut distance = 0;
    let mut frontier = BinaryHeap::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    frontier.push((Reverse(0), start));
    while let Some((Reverse(k), (i, j))) = frontier.pop() {
        if !visited.insert((i, j)) {
            continue;
        }
        distance = distance.max(k);
        let tile = &grid[i][j];
        if tile.north {
            frontier.push((Reverse(k + 1), (i - 1, j)));
        }
        if tile.south {
            frontier.push((Reverse(k + 1), (i + 1, j)));
        }
        if tile.west {
            frontier.push((Reverse(k + 1), (i, j - 1)));
        }
        if tile.east {
            frontier.push((Reverse(k + 1), (i, j + 1)));
        }
    }
    println!("{}", distance);

    // Part 2
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if !visited.contains(&(i, j)) {
                grid[i][j] = Default::default();
            }
        }
    }
    let mut exp_grid: Vec<Vec<Tile>> = grid
        .iter()
        .flat_map(|row| {
            [
                row.iter().flat_map(|tile| tile.expand()[0]).collect(),
                row.iter().flat_map(|tile| tile.expand()[1]).collect(),
            ]
        })
        .collect();

    let mut frontier: VecDeque<(usize, usize)> = VecDeque::new();
    let mut outsiders = 0;

    for i in 0..grid.len() {
        let exp_i = 2 * i;
        frontier.push_back((exp_i, 0));
        frontier.push_back((exp_i, exp_grid[exp_i].len() - 1));
    }
    for j in 0..grid[0].len() {
        let exp_j = 2 * j;
        frontier.push_back((0, exp_j));
        frontier.push_back((exp_grid.len() - 1, exp_j));
    }

    while let Some((i, j)) = frontier.pop_front() {
        if exp_grid[i][j] != Default::default() {
            continue;
        }
        exp_grid[i][j].outside = true;

        if i % 2 == 0 && j % 2 == 0 {
            outsiders += 1;
        }

        if i > 0 {
            frontier.push_back((i - 1, j));
        }
        if i < exp_grid.len() - 1 {
            frontier.push_back((i + 1, j));
        }
        if j > 0 {
            frontier.push_back((i, j - 1));
        }
        if j < exp_grid[i].len() - 1 {
            frontier.push_back((i, j + 1));
        }
    }

    let insiders = grid.len() * grid[0].len() - outsiders - visited.len();

    println!("{insiders}");
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct Tile {
    north: bool,
    east: bool,
    south: bool,
    west: bool,
    start: bool,
    outside: bool,
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '|' => Self {
                north: true,
                south: true,
                ..Default::default()
            },
            '-' => Self {
                east: true,
                west: true,
                ..Default::default()
            },
            'L' => Self {
                north: true,
                east: true,
                ..Default::default()
            },
            'J' => Self {
                north: true,
                west: true,
                ..Default::default()
            },
            '7' => Self {
                south: true,
                west: true,
                ..Default::default()
            },
            'F' => Self {
                south: true,
                east: true,
                ..Default::default()
            },
            'S' => Self {
                start: true,
                ..Default::default()
            },
            '.' => Default::default(),
            _ => panic!("{:?}", c),
        }
    }

    fn expand(&self) -> [[Tile; 2]; 2] {
        [
            [
                *self,
                Tile {
                    west: self.east,
                    east: self.east,
                    ..Default::default()
                },
            ],
            [
                Tile {
                    north: self.south,
                    south: self.south,
                    ..Default::default()
                },
                Default::default(),
            ],
        ]
    }
}