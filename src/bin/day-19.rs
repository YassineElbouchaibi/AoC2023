use std::collections::HashMap;

use advent_of_code_2023::utils::get_aoc_input_lines;
use itertools::Itertools;

fn main() {
    let input_lines = get_aoc_input_lines().expect("Error getting input");
    let workflows: HashMap<String, Workflow> = input_lines
        .map(|line| line.expect("Error parsing line"))
        .take_while(|line| !line.is_empty())
        .map(|line| Workflow::from_str(&line))
        .map(|workflow| (workflow.id.clone(), workflow))
        .collect();
    
    let input_lines = get_aoc_input_lines().expect("Error getting input");
    let total_rating: i64 = input_lines
        .map(|line| line.expect("Error parsing line"))
        .skip_while(|line| !line.is_empty())
        .skip_while(|line| line.is_empty())
        .map(|line| PartRatings::from_str(&line))
        .filter(|ratings| ratings.is_valid_part(&workflows, "in".to_string()))
        .map(|ratings| ratings.get_total_rating())
        .sum();
    println!("Total rating: {}", total_rating);

    let total_combinations = PartRangeRatings::new().count_valid_ranges(&workflows, "in".to_string());
    println!("Total combinations: {}", total_combinations);
}

#[derive(Debug)]
struct Workflow {
    id: String,
    rules: Vec<Rule>,
}

#[derive(Debug)]
struct Rule {
    condition: Option<Condition>,
    outcome: Outcome,
}

#[derive(Debug)]
struct Condition {
    category: Category,
    operator: Operator,
    value: i64,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Category {
    X,
    M,
    A,
    S,
}

#[derive(Debug)]
enum Operator {
    GreaterThan,
    LessThan,
}

#[derive(Debug)]
enum Outcome {
    GoTo(String),
    Reject,
    Accept,
}

impl Workflow {
    fn from_str(input_line: &str) -> Self {
        let mut parts = input_line.trim_end_matches("}").split("{");

        let id = parts.next().expect("Error parsing workflow id").to_string();
        
        let rules = parts.next().expect("Error parsing workflow rules");
        let rules = rules.split(",").map(|rule| Rule::from_str(rule)).collect();

        Self {
            id,
            rules,
        }
    }
}

impl Rule {
    fn from_str(input_line: &str) -> Self {
        let parts = input_line.split(":").collect_vec();

        if parts.len() == 1 {
            return Self {
                condition: None,
                outcome: Outcome::from_str(parts[0]),
            }
        }

        let condition = Condition::from_str(parts[0]);
        let outcome = Outcome::from_str(parts[1]);
        Self {
            condition: Some(condition),
            outcome,
        
        }
    }
}

impl Condition {
    fn from_str(input_line: &str) -> Self {
        let parts = input_line.split(&['<', '>']).collect_vec();
        let category = Category::from_str(parts[0]);
        let operator = Operator::from_str(input_line);
        let value = parts[1].parse::<i64>().expect("Error parsing value");

        Self {
            category,
            operator,
            value,
        }
    }

    fn is_met(&self, rating: i64) -> bool {
        match self.operator {
            Operator::GreaterThan => rating > self.value,
            Operator::LessThan => rating < self.value,
        }
    }
}

impl Category {
    fn from_str(input_line: &str) -> Self {
        match input_line {
            "x" => Self::X,
            "m" => Self::M,
            "a" => Self::A,
            "s" => Self::S,
            _ => panic!("Error parsing category for line {}", input_line),
        }
    }
}

impl Outcome {
    fn from_str(input_line: &str) -> Self {
        match input_line {
            "A" => Self::Accept,
            "R" => Self::Reject,
            _ => Self::GoTo(input_line.to_string()),
        }
    }
}

impl Operator {
    fn from_str(input_line: &str) -> Self {
        if input_line.contains(">") {
            Self::GreaterThan
        } else if input_line.contains("<") {
            Self::LessThan
        } else {
            panic!("Error parsing operator for line {}", input_line);
        }
    }
}

#[derive(Debug)]
struct PartRatings {
    ratings: HashMap<Category, i64>,
}

impl PartRatings {
    fn from_str(input_line: &str) -> Self {
        let ratings = input_line
            .trim_matches(&['{', '}'][..])
            .split(",")
            .map(|rating| rating.split("=").collect_tuple().unwrap())
            .map(|(c, v)| (
                Category::from_str(c),
                v.parse::<i64>().expect("Error parsing rating")
            ))
            .collect();
        
        Self {
            ratings,
        }
    }

    fn is_valid_part(&self, workflows: &HashMap<String, Workflow>, workflow_key: String) -> bool {
        let workflow = workflows.get(&workflow_key).expect("Error getting workflow");
        
        for rule in workflow.rules.iter() {
            if let Some(condition) = &rule.condition {
                let rating = self.ratings.get(&condition.category).expect("Error getting rating");
                if !condition.is_met(*rating) {
                    continue;
                }
            }

            match &rule.outcome {
                Outcome::Accept => return true,
                Outcome::Reject => return false,
                Outcome::GoTo(workflow_key) => return self.is_valid_part(workflows, workflow_key.clone()),
            }
        }

        panic!("Error: no valid outcome found for part");
    }

    fn get_total_rating(&self) -> i64 {
        self.ratings.values().sum()
    }
}

#[derive(Debug, Clone)]
struct PartRangeRatings {
    ratings: HashMap<Category, (i64, i64)>,
}

impl PartRangeRatings {
    fn new() -> Self {
        Self {
            ratings: HashMap::from([
                (Category::X, (1, 4000)),
                (Category::M, (1, 4000)),
                (Category::A, (1, 4000)),
                (Category::S, (1, 4000)),
            ]),
        }
    }

    fn count_valid_ranges(&self, workflows: &HashMap<String, Workflow>, workflow_key: String) -> i64 {
        let workflow = workflows.get(&workflow_key).expect("Error getting workflow");
        
        let mut range = self.clone();
        let mut counter = 0;
        for rule in workflow.rules.iter() {
            let mut opposite_range = range.clone();

            if let Some(condition) = &rule.condition {
                let rating = range.ratings.get_mut(&condition.category).expect("Error getting rating");
                match condition.operator {
                    Operator::GreaterThan => rating.0 = rating.0.max(condition.value + 1),
                    Operator::LessThan => rating.1 = rating.1.min(condition.value - 1),
                }

                let opposite_rating = opposite_range.ratings.get_mut(&condition.category).expect("Error getting rating");
                match condition.operator {
                    Operator::GreaterThan => opposite_rating.1 = opposite_rating.1.min(condition.value),
                    Operator::LessThan => opposite_rating.0 = opposite_rating.0.max(condition.value),
                }
            }

            match &rule.outcome {
                Outcome::Accept => counter += range.get_total_combinations(),
                Outcome::Reject => counter += 0,
                Outcome::GoTo(workflow_key) => counter += range.count_valid_ranges(workflows, workflow_key.clone()),
            }

            range = opposite_range;
        }

        return counter;
    }

    fn get_total_combinations(&self) -> i64 {
        self.ratings.values().map(|(min, max)| max - min + 1).product()
    }
}