use advent_of_code_2023::utils::get_aoc_input_lines;

#[derive(Debug)]
struct Game {
    id: usize,
    reveals: Vec<Reveal>,
}

#[derive(Debug)]
struct Reveal {
    red: usize,
    green: usize,
    blue: usize,
}

const VALID_REVEAL_CEILINGS: Reveal = Reveal {
    red: 12,
    green: 13,
    blue: 14,
};

fn main() {
    let input_lines = get_aoc_input_lines().expect("Error getting input");
    let mut sum_valid_game_ids = 0;
    let mut sum_min_reveal_power = 0;

    for line in input_lines {
        let line = line.expect("Error reading line");

        let game = Game::from_input(&line);

        if game.is_valid() {
            sum_valid_game_ids += game.id;
        }

        sum_min_reveal_power += game.max_reveal_power();
    }

    println!("Part A: {}", sum_valid_game_ids);
    println!("Part B: {}", sum_min_reveal_power);
}

impl Game {
    fn from_input(line: &String) -> Game {
        // Get the id
        let id = line.split(":").next().expect("Error getting id");
        let id = id.split(" ").last().expect("Error getting id");
        let id = id.parse::<usize>().expect("Error parsing id");

        // Get the reveals
        let reveals = line.split(":").last().expect("Error getting reveals");
        let reveals = reveals.split(";").map(Reveal::from_input).into_iter();

        Game {
            id,
            reveals: reveals.collect(),
        }
    }

    fn is_valid(&self) -> bool {
        self.reveals.iter().all(|reveal| reveal.is_valid())
    }

    fn max_reveal(&self) -> Reveal {
        let mut max_reveal = Reveal {
            red: 0,
            green: 0,
            blue: 0,
        };

        self.reveals.iter().for_each(|reveal| {
            if reveal.red > max_reveal.red {
                max_reveal.red = reveal.red;
            }
            if reveal.green > max_reveal.green {
                max_reveal.green = reveal.green;
            }
            if reveal.blue > max_reveal.blue {
                max_reveal.blue = reveal.blue;
            }
        });

        max_reveal
    }

    fn max_reveal_power(&self) -> usize {
        self.max_reveal().power()
    }
}



impl Reveal {
    fn from_input(line: &str) -> Reveal {
        let mut reveal = Reveal {
            red: 0,
            green: 0,
            blue: 0,
        };

        line.split(",").for_each(|color_input| {
            let count = color_input
                .trim()
                .split(" ")
                .next()
                .expect("Error getting color count");
            let count = count.parse::<usize>().expect("Error parsing color count");
            match color_input.split(" ").last().expect("Error getting color") {
                "red" => reveal.red = count,
                "green" => reveal.green = count,
                "blue" => reveal.blue = count,
                _ => panic!("Error parsing color"),
            };
        });

        reveal
    }

    fn is_valid(&self) -> bool {
        self.red <= VALID_REVEAL_CEILINGS.red
            && self.green <= VALID_REVEAL_CEILINGS.green
            && self.blue <= VALID_REVEAL_CEILINGS.blue
    }

    fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}