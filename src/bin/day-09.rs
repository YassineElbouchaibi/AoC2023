use advent_of_code_2023::utils::get_aoc_input_lines;

fn main() {
    let input_lines = get_aoc_input_lines().expect("Error getting input");

    let sum_extrapolated_values = input_lines
        .map(|line| parse_line(&line.expect("Error reading line")))
        .map(|history| extrapolate_next(&history))
        .map(|history| extrapolate_previous(&history))
        .map(|extrapolated| (extrapolated.first().unwrap().clone(), extrapolated.last().unwrap().clone()))
        .fold((0, 0), |(s_i, f_i), (s, f)| (s_i + s, f_i + f));
    println!("Sum of extrapolated values (prev, next): {:?}", sum_extrapolated_values);
}

fn parse_line(line: &str) -> Vec<i64> {
    line.split_whitespace()
        .map(|s| s.parse::<i64>().expect("Error parsing number"))
        .collect()
}

fn extrapolate_next(history: &Vec<i64>) -> Vec<i64> {
    let mut history = history.clone();
    let differentials = history.differentiate();
    
    let last_value = history.last().unwrap();
    let end_reached = differentials.iter().all(|&d| d == 0);

    if end_reached {
        history.push(*last_value);
    } else {
        history.push(*last_value + extrapolate_next(&differentials).last().unwrap());
    }

    history
}

fn extrapolate_previous(history: &Vec<i64>) -> Vec<i64> {
    let mut history = history.clone();
    let differentials = history.differentiate();
    
    let first_value = history.first().unwrap();
    let end_reached = differentials.iter().all(|&d| d == 0);

    if end_reached {
        history.insert(0, *first_value);
    } else {
        history.insert(0, *first_value - extrapolate_previous(&differentials).first().unwrap());
    }

    history
}

trait Differentiable {
    fn differentiate(&self) -> Self;
}

impl Differentiable for Vec<i64> {
    fn differentiate(&self) -> Self {
        self.windows(2)
            .map(|w| w[1] - w[0])
            .collect()
    }
}