use std::collections::HashMap;

use advent_of_code_2023::utils::get_aoc_input_lines;
use itertools::Itertools;

fn main() {
    let input_lines = get_aoc_input_lines().expect("Error getting input");
    let hands = input_lines
        .map(|line| Hand::from_str(&line.expect("Error reading input line")))
        .sorted_by_key(|hand| (hand.hand_type, hand.cards.clone()))
        .enumerate()
        .inspect(|hand| println!("{:?}", hand))
        .fold(0, |acc, (i, hand)| {
            let rank = i + 1;
            acc + hand.bid * rank
        });

    println!("Total bid: {}", hands);
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
#[repr(u8)]
enum Card {
    Joker = 0,
    Number(u8),
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

impl Card {
    #[allow(unreachable_patterns)]
    fn from_str(s: &str) -> Card {
        match s {
            "J" => Card::Joker,
            "J" => Card::Jack, // Used in Part 1, unreachable in Part 2
            "Q" => Card::Queen,
            "K" => Card::King,
            "A" => Card::Ace,
            "T" => Card::Number(10),
            _ => Card::Number(s.parse::<u8>().expect("Error parsing card value")),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
#[repr(u8)]
enum HandType {
    FiveOfAKind = 6, // Strongest
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0, // Weakest
}

impl HandType {
    fn from_cards(cards: &Vec<Card>) -> HandType {
        let mut counter = HashMap::new();
        for card in cards {
            *counter.entry(card).or_insert(0) += 1;
        }

        // Add jokers to most common card
        let jokers = *counter.get(&Card::Joker).unwrap_or(&0);
        if jokers > 0 && jokers < 5 {
            counter.remove(&Card::Joker);
            let max_card = counter
                .iter()
                .max_by_key(|(_, &v)| v)
                .expect("Error getting max card").0;
            *counter.entry(&max_card).or_insert(0) += jokers;
        }

        if counter.len() == 5 {
            HandType::HighCard
        } else if counter.len() == 4 {
            HandType::OnePair
        } else if counter.len() == 3 {
            if counter.values().any(|&v| v == 3) {
                HandType::ThreeOfAKind
            } else {
                HandType::TwoPair
            }
        } else if counter.len() == 2 {
            if counter.values().any(|&v| v == 4) {
                HandType::FourOfAKind
            } else {
                HandType::FullHouse
            }
        } else {
            HandType::FiveOfAKind
        }
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: usize,
    hand_type: HandType,
}

impl Hand {
    fn from_str(s: &str) -> Hand {
        let mut s = s.trim().split_whitespace();
        let cards: Vec<Card> = s
            .next()
            .expect("Error parsing cards from hand line")
            .chars()
            .map(|c| Card::from_str(&c.to_string()))
            .collect();
        let bid = s
            .next()
            .expect("Error parsing bid from hand line")
            .parse::<usize>()
            .expect("Error parsing bid from hand line");
        let hand_type = HandType::from_cards(&cards);

        Hand {
            cards,
            bid,
            hand_type,
        }
    }
}
