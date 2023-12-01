use std::fmt::format;
use std::fs;
use std::env;

fn main() {
    let args : Vec<String> = env::args().collect();
    let arg : &String = args.iter().nth(1).expect("no filepath was specified");
    let lines = read_lines(arg);
    let result : u32 = lines.iter().map(|line| get_line_number(&line)).sum();
    println!("the result is {}", result);
}

fn read_lines(filename : &str) -> Vec<String> {
    fs::read_to_string(filename).expect("invalid filepath given").lines().map(String::from).collect()
}

fn get_line_number(line : &str) -> u32 {
    let mut numbers = line.chars().filter_map(|a| a.to_digit(10));
    let first = numbers.next().expect(&format!("no numbers were found in line {}", line));
    let last = match numbers.last() {
        Some(number) => number,
        None => first,
    };
    return first * 10 + last;
}
