use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let filepath = env::args().nth(1).unwrap();
    let blocks: Vec<Vec<String>> = fs::read_to_string(filepath)
        .unwrap()
        .lines()
        .map(String::from)
        .collect::<Vec<String>>()
        .split(|line| line.is_empty())
        .map(Vec::from)
        .collect();
    //let result : u32 = blocks.iter().map(|block| vertical_axis(block) + 100 * horizontal_axis(block)).sum();
    let result: u32 = blocks.iter().map(|block| vertical_axis(block)).sum::<u32>()
        + 100 * blocks.iter().map(horizontal_axis).sum::<u32>();
    dbg!(result);
}

fn vertical_axis(block: &[String]) -> u32 {
    let mut possibilities: HashSet<usize> = vertical_possibilities(&block[0]);
    for line in block.iter().skip(1) {
        let chars: Vec<char> = line.chars().collect();
        let mut buffer_set: HashSet<usize> = HashSet::new();
        for axis in possibilities.iter() {
            if chars
                .iter()
                .take(*axis)
                .enumerate()
                .map(|(pos, c)| match chars.get(axis + (axis - pos - 1)) {
                    None => true,
                    Some(other) => c == other,
                })
                .all(|a| a)
            {
                buffer_set.insert(*axis);
            }
        }
        possibilities = possibilities.intersection(&buffer_set).copied().collect();
        if possibilities.is_empty() {
            break;
        }
    }
    possibilities.iter().copied().sum::<usize>() as u32
}

fn vertical_possibilities(line: &str) -> HashSet<usize> {
    let chars: Vec<char> = line.chars().collect();
    let mut possibilities: HashSet<usize> = HashSet::new();
    for axis in 1..(chars.len()) {
        if chars
            .iter()
            .take(axis)
            .enumerate()
            .map(|(pos, c)| match chars.get(axis + (axis - pos - 1)) {
                None => true,
                Some(other) => c == other,
            })
            .all(|a| a)
        {
            possibilities.insert(axis);
        }
    }
    possibilities
}

fn horizontal_axis(block: &Vec<String>) -> u32 {
    (1..(block.len()))
        .filter(|axis| {
            block
                .iter()
                .take(*axis)
                .enumerate()
                .map(|(pos, line)| match block.get(axis + (axis - pos - 1)) {
                    None => true,
                    Some(s) => s == line,
                })
                .all(|a| a)
        })
        .sum::<usize>() as u32
}
