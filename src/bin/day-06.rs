use advent_of_code_2023::utils::get_aoc_input_lines;

fn main() {
    let mut input_lines = get_aoc_input_lines().unwrap();
    let times = parse_line(&input_lines.next().unwrap().unwrap());
    let records = parse_line(&input_lines.next().unwrap().unwrap());
    let power = times.iter().zip(records.iter()).map(|(x, y)| {
        get_winning_distances_count(*x, *y)
    }).fold(1, |acc, x| acc * x);
    println!("Power: {}", power);
}

fn parse_line(line: &str) -> Vec<i64> {
    let line = line.split(":").last().expect("Error parsing line").trim();
    let line = line.replace(" ", "");
    line.split_whitespace().map(|x| x.parse::<i64>().expect("Error parsing line")).collect()
}

fn get_winning_distances_count(time: i64, record: i64) -> i64 {
    let mut count = 0;
    let mut we_are_now_above = false;
    for i in 0..time {
        let distance = i * (time - i);
        if distance > record {
            count += 1;
            we_are_now_above = true;
        } else if we_are_now_above {
            break;
        }
    }

    println!("Time: {}, Record: {}, Count: {}", time, record, count);

    count
}