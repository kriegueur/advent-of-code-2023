use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use regex::Regex;

fn main() {
    let filepath : String = env::args().nth(1).expect("missing argument for filepath");
    let lines = read_lines(&filepath);
    /* Part 1
    let total_score : u32 = lines.iter().map(|l| get_card_points(l)).sum();
    print!("{}", total_score);
    */
    let mut card_scores : HashMap<usize, u32> = HashMap::new();
    let mut card_amounts : Vec<u32> = vec![];
    for (index, card) in lines.iter().enumerate() {
        card_scores.insert(index + 1, get_card_wins(card));
        card_amounts.push(1);
    };
    for i in 1..=lines.len() {
        let card_score : u32 = card_scores.get(&i).unwrap().to_owned();
        for j in 1..=card_score as usize {
            card_amounts[i + j - 1] += card_amounts[i - 1];
        }
    };
    let result : u32 = card_amounts.iter().sum();
    println!("{}", result);
}

fn read_lines(filepath : &str) -> Vec<String> {
    fs::read_to_string(filepath).expect("invalid filepath").lines().map(String::from).collect()
}

/* Part 1
fn get_card_points(card : &str) -> u32 {
    let split_re = Regex::new(r"Card +\d+: ([\d ]+) \| ([\d ]+)").unwrap();
    let split_caps = split_re.captures(card).expect(card);
    let winning_numbers : String = split_caps[1].into();
    let pulled_numbers : String = split_caps[2].into();
    let numbers_re = Regex::new(r"\d+").unwrap();
    let winning_set : HashSet<u32> = HashSet::from_iter(numbers_re.find_iter(&winning_numbers).map(|n| n.as_str().parse().expect("this should not happen")));
    let score = numbers_re.find_iter(&pulled_numbers).filter(|n| winning_set.contains(&n.as_str().parse().expect("this should not happen"))).fold(0, |a, _b| match a {
        0 => 1,
        x => x*2,
    });
    score
}
*/

fn get_card_wins(card : &str) -> u32 {
    let split_re = Regex::new(r"Card +\d+: ([\d ]+) \| ([\d ]+)").unwrap();
    let split_caps = split_re.captures(card).expect(card);
    let winning_numbers : String = split_caps[1].into();
    let pulled_numbers : String = split_caps[2].into();
    let numbers_re = Regex::new(r"\d+").unwrap();
    let winning_set : HashSet<u32> = HashSet::from_iter(numbers_re.find_iter(&winning_numbers).map(|n| n.as_str().parse().expect("this should not happen")));
    let score = numbers_re.find_iter(&pulled_numbers).filter(|n| winning_set.contains(&n.as_str().parse().expect("this should not happen"))).count() as u32;
    score
}
