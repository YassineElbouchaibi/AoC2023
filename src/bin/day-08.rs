use std::collections::HashMap;

use advent_of_code_2023::utils::get_aoc_input_lines;

fn main() {
    let mut input_lines = get_aoc_input_lines().expect("Error getting input");

    let instructions: Vec<Instruction> = input_lines
        .next()
        .expect("Error reading next instructions line")
        .expect("Error reading instructions line")
        .chars()
        .map(|s| Instruction::from_str(&s.to_string()))
        .collect();

    let node_map: HashMap<String, Node> = input_lines
        .skip_while(|line| line.as_ref().expect("Error reading line").trim().is_empty())
        .map(|line| {
            let node = Node::from_str(&line.expect("Error reading line"));
            (node.key.clone(), node)
        })
        .collect();

    let mut lcm = 1;
    let instructions_count = instructions.len();
    let mut steps_count = 0;
    let mut current_nodes = node_map
        .iter()
        .filter(|(_, node)| node.is_start_node())
        .map(|(_, node)| node)
        .collect::<Vec<&Node>>();
    loop {
        let instruction = instructions
            .get(steps_count % instructions_count)
            .expect("Error getting instruction");
        steps_count += 1;

        current_nodes = current_nodes
            .iter()
            .map(|node| node_map.get(node.get_next_node(instruction)).expect("Error getting next node"))
            .collect::<Vec<&Node>>();

        if current_nodes.iter().any(|node| node.is_end_node()) {
            lcm = num::integer::lcm(lcm, steps_count);
        }

        current_nodes = current_nodes
            .iter()
            .filter(|node| !node.is_end_node())
            .map(|&node| node)
            .collect::<Vec<&Node>>();

        if current_nodes.len() == 0 {
            break;
        }
    }

    println!("LCM: {:?}", lcm);
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Instruction {
    Left,
    Right,
}

impl Instruction {
    fn from_str(s: &str) -> Instruction {
        match s {
            "L" => Instruction::Left,
            "R" => Instruction::Right,
            _ => panic!("Error parsing instruction"),
        }
    }
}

#[derive(Debug)]
struct Node {
    key: String,
    next: HashMap<Instruction, String>,
}

impl Node {
    fn from_str(s: &str) -> Node {
        let mut parts = s.trim().split("=");
        let key = parts
            .next()
            .expect("Error parsing node key")
            .trim()
            .to_string();

        let parts = parts.next().expect("Error parsing node value");
        let parts = parts.replace("(", "");
        let parts = parts.replace(")", "");
        let parts = parts.replace(" ", "");
        let mut parts = parts.split(",");
        let left = parts
            .next()
            .expect("Error parsing left node")
            .trim()
            .to_string();
        let right = parts
            .next()
            .expect("Error parsing right node")
            .trim()
            .to_string();

        Node {
            key,
            next: HashMap::from([(Instruction::Left, left), (Instruction::Right, right)]),
        }
    }

    fn get_next_node(&self, instruction: &Instruction) -> &String {
        self.next.get(instruction).expect("Error getting next node")
    }

    fn is_start_node(&self) -> bool {
        self.key.ends_with("A")
    }

    fn is_end_node(&self) -> bool {
        self.key.ends_with("Z")
    }
}
