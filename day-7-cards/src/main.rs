use itertools::Itertools;
use std::collections::HashMap;
use std::env;
use std::fs;

const FIVE: u32 = 7;
const FOUR: u32 = 6;
const FULL: u32 = 5;
const THREE: u32 = 4;
const TWOPAIRS: u32 = 3;
const PAIR: u32 = 2;
const ONES: u32 = 1;

fn main() {
    let filepath: String = env::args()
        .nth(1)
        .expect("expected at least one argument for filepath");
    let lines = fs::read_to_string(filepath)
        .expect("invalid filepath")
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();
    let mut hands: Vec<Hand> = lines
        .iter()
        .map(|line| line_to_hand(line))
        .collect::<Vec<Hand>>();
    hands.sort();
    let result: u32 = hands
        .iter()
        .enumerate()
        .map(|(index, hand)| (index + 1) as u32 * hand.bet)
        .sum();
    dbg!(result);
}

fn line_to_hand(line: &str) -> Hand {
    let mut split_line = line.split(' ');
    let cards = split_line.next().unwrap();
    let bet: u32 = split_line.next().unwrap().parse().unwrap();
    HandBuilder::new(cards).bet(bet).build()
}

const fn strength(c: char) -> u32 {
    match c {
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        '9' => 8,
        'T' => 9,
        'J' => 10,
        'Q' => 11,
        'K' => 12,
        'A' => 13,
        _ => 0,
    }
}

#[derive(Debug)]
struct Hand {
    cards: String,
    bet: u32,
    strength: u32,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards && self.bet == other.bet
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.strength.cmp(&other.strength) {
            std::cmp::Ordering::Equal => {
                for (me, you) in self.cards.chars().zip(other.cards.chars()) {
                    let cmp = strength(me).cmp(&strength(you));
                    match cmp {
                        std::cmp::Ordering::Equal => continue,
                        _ => return cmp,
                    };
                }
                std::cmp::Ordering::Equal
            }
            x => x,
        }
    }
}

struct HandBuilder<'a> {
    cards: &'a str,
    bet: u32,
    strength: u32,
}

impl<'a> HandBuilder<'a> {
    fn new(cards: &str) -> HandBuilder {
        let mut sorted_cards: HashMap<char, u32> = HashMap::new();
        for card in cards.chars() {
            *sorted_cards.entry(card).or_insert(0) += 1;
        }
        let mut amounts = sorted_cards.values().sorted().rev();
        let strength: u32 = match amounts.next().unwrap() {
            5 => FIVE,
            4 => FOUR,
            3 => match amounts.next().unwrap() {
                2 => FULL,
                1 => THREE,
                _ => panic!("how"),
            },
            2 => match amounts.next().unwrap() {
                2 => TWOPAIRS,
                1 => PAIR,
                _ => panic!("how"),
            },
            1 => ONES,
            _ => panic!("how"),
        };
        HandBuilder {
            cards,
            bet: 0,
            strength,
        }
    }

    fn bet(mut self, bet: u32) -> HandBuilder<'a> {
        self.bet = bet;
        self
    }

    fn build(&self) -> Hand {
        Hand {
            cards: String::from(self.cards),
            bet: self.bet,
            strength: self.strength,
        }
    }
}
