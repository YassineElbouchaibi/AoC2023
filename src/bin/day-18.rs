use advent_of_code_2023::utils::get_aoc_input_lines;

fn main() {
    let input_lines = get_aoc_input_lines().expect("Error getting input");

    let mut points_part_1 = Vec::new();
    let mut points_part_2 = Vec::new();
    let mut current_node_part_1 = (0, 0);
    let mut current_node_part_2 = (0, 0);

    points_part_1.push(current_node_part_1);
    points_part_2.push(current_node_part_2);

    for line in input_lines {
        let line = line.expect("Error reading line");
        let mut parts = line.split_whitespace();
        let direction = parts.next().expect("Error getting direction");
        let distance: i64 = parts.next().expect("Error getting distance").parse().expect("Error parsing distance");
        let hex_code = parts.next().expect("Error getting color").trim_start_matches("(").trim_end_matches(")").trim_start_matches("#");

        match direction {
            "U" => current_node_part_1.1 += distance,
            "D" => current_node_part_1.1 -= distance,
            "L" => current_node_part_1.0 -= distance,
            "R" => current_node_part_1.0 += distance,
            _ => panic!("Unknown direction: {}", direction),
        }
        points_part_1.push(current_node_part_1);

        let direction = &hex_code[5..=5];
        let distance = i64::from_str_radix(&hex_code[0..5], 16).expect("Error parsing distance");

        match direction {
            "3" => current_node_part_2.1 += distance,
            "1" => current_node_part_2.1 -= distance,
            "2" => current_node_part_2.0 -= distance,
            "0" => current_node_part_2.0 += distance,
            _ => panic!("Unknown direction: {}", direction),
        }
        points_part_2.push(current_node_part_2);
    }

    println!("Area - Part 1: {}", compute_area(&points_part_1));
    println!("Area - Part 2: {}", compute_area(&points_part_2));
}

fn compute_area(points: &Vec<(i64, i64)>) -> i64 {
    // Gauss's shoelace formula
    let mut area = 0;
    let mut perimeter = 0;

    for i in 0..(points.len() - 1) {
        let (x1, y1) = points[i];
        let (x2, y2) = points[i + 1];
        area += x1 * y2 - y1 * x2;
        perimeter += num::abs(x2 - x1) + num::abs(y2 - y1);
    }
    area = num::abs(area) / 2;

    // Pick's theorem
    let interior = area - perimeter / 2 + 1;

    interior + perimeter
}