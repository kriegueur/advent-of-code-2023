use std::env;
use std::fs;

fn main() {
    let filepath = env::args().nth(1).unwrap();
    let file_content = fs::read_to_string(&filepath).unwrap();
    let result : u32 = file_content.lines().map(line_configurations).sum();
    dbg!(result);
}

enum Spring {
    Good,
    Damaged,
    Unknown,
}

fn line_configurations(line : &str) -> u32 {
    let mut parts = line.split(' ');
    let springs : Vec<Spring> = parts.next().unwrap().chars().map(|c| match c {
        '.' => Spring::Good,
        '#' => Spring::Damaged,
        _ => Spring::Unknown,
    }).collect();
    let predicate : Vec<u32> = parts.next().unwrap().split(',').map(|c| c.parse().unwrap()).collect();
    let mut state : Vec<u32> = vec![];
    possibilities_from_position(&springs, 0, 0, &predicate, &mut state)
}

fn possibilities_from_position(springs : &Vec<Spring>, pos : u32, damaged_found : u32, predicate : &Vec<u32>, state : &mut Vec<u32>) -> u32 {
    match springs.get(pos as usize) {
        None => {
            if damaged_found != 0 {
                state.push(damaged_found);
                match state.iter().cmp(predicate.iter().take(state.len())) {
                    std::cmp::Ordering::Equal => (),
                    _ => return 0,
                };
            }
            match state.iter().cmp(predicate.iter()) {
            std::cmp::Ordering::Equal => 1,
            _ => 0,
            }
        },
        Some(spring) => {
            match spring {
                Spring::Good => {
                    if damaged_found != 0 {
                        state.push(damaged_found);
                        match state.iter().cmp(predicate.iter().take(state.len())) {
                            std::cmp::Ordering::Equal => (),
                            _ => return 0,
                        };
                    }
                    possibilities_from_position(springs, pos + 1, 0, predicate, state)
                },
                Spring::Damaged => {
                    possibilities_from_position(springs, pos + 1, damaged_found + 1, predicate, state)
                },
                Spring::Unknown => {
                    let mut state_damaged = state.iter().copied().collect();
                    let mut undamaged : u32 = 1;
                    if damaged_found != 0 {
                        state.push(damaged_found);
                        match state.iter().cmp(predicate.iter().take(state.len())) {
                            std::cmp::Ordering::Equal => (),
                            _ => undamaged = 0,
                        };
                    }
                    undamaged * possibilities_from_position(springs, pos + 1, 0, predicate, state) + possibilities_from_position(springs, pos + 1, damaged_found + 1, predicate, &mut state_damaged)
                }
            }
        },
    }
}
