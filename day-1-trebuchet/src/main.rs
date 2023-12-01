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
    let mut number_vec : Vec<u32> = vec![];
    for i in 0..line.len() {
        if line[i..].starts_with("one") {
            number_vec.push(1);
        }
        else if line[i..].starts_with("two") {
            number_vec.push(2);
        }
        else if line[i..].starts_with("three") {
            number_vec.push(3);
        }
        else if line[i..].starts_with("four") {
            number_vec.push(4);
        }
        else if line[i..].starts_with("five") {
            number_vec.push(5);
        }
        else if line[i..].starts_with("six") {
            number_vec.push(6);
        }
        else if line[i..].starts_with("seven") {
            number_vec.push(7);
        }
        else if line[i..].starts_with("eight") {
            number_vec.push(8);
        }
        else if line[i..].starts_with("nine") {
            number_vec.push(9);
        }
        else {
            match line.chars().nth(i).unwrap().to_digit(10) {
                Some(x) => number_vec.push(x),
                _ => (),
            }
        }
    }

    let mut numbers = number_vec.iter();
    let first = numbers.next().expect(&format!("no numbers were found in line {}", line));
    let last = match numbers.last() {
        Some(number) => number,
        None => first,
    };
    return first * 10 + last;
}
