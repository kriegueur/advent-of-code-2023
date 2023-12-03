use std::char;
use std::env;
use std::fs;

fn main() {
    let args : Vec<String> = env::args().collect();
    let arg : &String = args.iter().nth(1).expect("no filepath was specifed");
    let lines = read_lines(arg);
    /* Part 1
    let numbers = get_numbers(&lines);
    let part_numbers : Vec<&Number> = numbers.iter().filter(|n| is_part_number(n, &lines)).collect();
    let result : u32 = part_numbers.iter().map(|n| get_number_value(n, &lines)).sum();
    println!("{}", result);
    */
    let gears = get_gears(&lines);
    let result : u32 = gears.iter().filter_map(|g| get_gear_ratio((g.0, g.1), &lines)).sum();
    println!("{}", result);
}

fn read_lines(filepath : &str) -> Vec<Vec<char>> {
    fs::read_to_string(filepath).expect("invalid filepath given").lines().map(|s| s.chars().collect()).collect()
}

#[derive(Debug)]
struct Number {
    pos : (usize, usize),
    length : usize,
}

fn get_numbers(board : &Vec<Vec<char>>) -> Vec<Number> {
    let max_x : usize = board[0].len();
    let max_y : usize = board.len();
    let mut x : usize = 0;
    let mut y : usize = 0;
    let mut numbers : Vec<Number> = vec![];
    while y < max_y {
        while x < max_x {
            if board[y][x].is_digit(10) {
                let pos = (y, x);
                while x < max_x && board[y][x].is_digit(10) {
                    x = x + 1;
                }
                let size = x - pos.1;
                numbers.push(Number { pos: pos, length: size })
            }
            else {
                x = x + 1;
            }
        }
        x = 0;
        y += 1;
    };
    numbers
}

fn get_gears(board : &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let max_x : usize = board[0].len();
    let max_y : usize = board.len();
    let mut x : usize = 0;
    let mut y : usize = 0;
    let mut gears : Vec<(usize, usize)> = vec![];
    while y < max_y {
        while x < max_x {
            if board[y][x] == '*' {
                gears.push((y,x));
            }
            x += 1;
        }
        x = 0;
        y += 1;
    };
    gears
}

fn is_part_number(number : &Number, board : &Vec<Vec<char>>) -> bool {
    let max_x : usize = board[0].len();
    let max_y : usize = board.len();
    let check_top = number.pos.0 > 0;
    let check_bottom = number.pos.0 + 1 < max_y;
    let check_left = number.pos.1 > 0;
    let check_right = number.pos.1 + number.length < max_x;
    //checking corners last
    //starting with left and right
    if check_left && is_symbol(board[number.pos.0][number.pos.1 - 1]) {
        return true;
    }
    if check_right && is_symbol(board[number.pos.0][number.pos.1 + number.length]) {
        return true;
    }
    // full above line
    if check_top {
        for x in number.pos.1..(number.pos.1 + number.length) {
            if is_symbol(board[number.pos.0 - 1][x]) {
                return true;
            }
        }
    }
    // full below check
    if check_bottom {
        for x in number.pos.1..(number.pos.1 + number.length) {
            if is_symbol(board[number.pos.0 + 1][x]) {
                return true;
            }
        }
    }
    //corners
    if check_top {
        if check_right {
            if is_symbol(board[number.pos.0 - 1][number.pos.1 + number.length]) {
                return true;
            }
        }
        if check_left {
            if is_symbol(board[number.pos.0 - 1][number.pos.1 - 1]) {
                return true;
            }
        }
    }
    if check_bottom {
        if check_right {
            if is_symbol(board[number.pos.0 + 1][number.pos.1 + number.length]) {
                return true;
            }
        }
        if check_left {
            if is_symbol(board[number.pos.0 + 1][number.pos.1 - 1]) {
                return true;
            }
        }
    }
    false
}

fn get_gear_ratio(gear : (usize, usize), board : &Vec<Vec<char>>) -> Option<u32> {
    let max_x : usize = board[0].len();
    let max_y : usize = board.len();
    let check_top = gear.0 > 0;
    let check_bottom = gear.0 + 1 < max_y;
    let check_left = gear.1 > 0;
    let check_right = gear.1 + 1 < max_x;
    // for top and bottom we want to check left to right
    // you might have something such as
    // 456
    // .*.
    // 22.
    let mut adjacent_numbers : Vec<Number> = vec![];
    //check above row
    if check_top {
        let mut x : usize = match check_left {
            true => gear.1 - 1,
            false => gear.1,
        };
        while x < max_x  && x < gear.1 + 2 {
            if !board[gear.0 - 1][x].is_digit(10) {
                x += 1;
            }
            else {
                let number = get_number_from_pos((gear.0 - 1, x), board);
                x = number.pos.1 + number.length;
                adjacent_numbers.push(number);
            }
        }
    };
    //check below row
    if check_bottom {
        let mut x : usize = match check_left {
            true => gear.1 - 1,
            false => gear.1,
        };
        while x < max_x && x < gear.1 + 2 {
            if !board[gear.0 + 1][x].is_digit(10) {
                x += 1;
            }
            else {
                let number = get_number_from_pos((gear.0 + 1, x), board);
                x = number.pos.1 + number.length;
                adjacent_numbers.push(number);
            }
        }
    };
    // check left
    if check_left  && board[gear.0][gear.1 - 1].is_digit(10) {
        adjacent_numbers.push(get_number_from_pos((gear.0, gear.1 - 1), board));
    };
    if check_right && board[gear.0][gear.1 + 1].is_digit(10){
        adjacent_numbers.push(get_number_from_pos((gear.0, gear.1 + 1), board));
    };
    if adjacent_numbers.len() != 2 {
        return None;
    };
    Some(adjacent_numbers.iter().map(|n| get_number_value(n, board)).product())
}

fn get_number_from_pos(gear : (usize, usize), board : &Vec<Vec<char>>) -> Number {
    let mut left = gear.1;
    let mut right = gear.1;
    while left > 0 && board[gear.0][left - 1].is_digit(10) {
        left -= 1;
    };
    while right < board[0].len() && board[gear.0][right].is_digit(10) {
        right += 1;
    };
    Number { pos: (gear.0, left), length: right - left }
}

fn is_symbol(c : char) -> bool {
    !c.is_digit(10) && c != '.'
}

fn get_number_value(number : &Number, board : &Vec<Vec<char>>) -> u32 {
    let mut result = 0;
    for x in number.pos.1..(number.pos.1 + number.length) {
        let digit = board[number.pos.0][x].to_digit(10).unwrap();
        result = result * 10 + digit;
    };
    result
}
