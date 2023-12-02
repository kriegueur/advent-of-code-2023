use std::collections::HashMap;
use std::fs;
use std::env;
use std::usize;

fn main() {
    let args : Vec<String> = env::args().collect();
    let arg : &String = args.iter().nth(1).expect("no filepath was specified");
    let lines = read_lines(arg);
    let games = lines.iter().map(|l| parse_game(l));
    /* part 1
    let valid_games = games.filter(|g| validate_game(&g));
    let result : u32 = valid_games.map(|g| g.id).sum();
    */
    let result : u32 = games.map(|g| game_power(&g)).sum();
    println!("{}", result);
}

fn read_lines(filename : &str) -> Vec<String> {
    fs::read_to_string(filename).expect("invalid filepath given").lines().map(String::from).collect()
}

struct Game {
    id : u32,
    counts : Vec<(u32, u32, u32)>,
}

fn advance_iter<T : Iterator>(iter : &mut T, size : usize) {
    for _i in 0..size {
        iter.next();
    }
}

fn parse_game(line : &String) -> Game {
    let mut char_iter = line.chars();
    let game_text = char_iter.clone().take("Game ".len()).collect::<String>();
    if !game_text.eq("Game ".into()) {
        panic!("invalid line format wanted Game found {}", game_text);
    }
    advance_iter(&mut char_iter, game_text.len());
    let id_text : String = char_iter.clone().take_while(|c| c.is_digit(10)).collect();
    advance_iter(&mut char_iter, id_text.len());
    let id : u32 = id_text.parse().expect("invalid id");
    if char_iter.next() != Some(':') {
        panic!("expected colon");
    }
    let mut rolls : Vec<String> = vec![];
    while char_iter.next() != None {
        let roll : String = char_iter.clone().take_while(|c| c.clone() != ';').collect();
        advance_iter(&mut char_iter, roll.len());
        rolls.push(roll.trim().into());
    };
    let parsed_rolls = rolls.iter().map(|r| parse_roll(&r)).collect();
    Game { id : id, counts : parsed_rolls }
}

fn parse_roll(roll : &String) -> (u32, u32, u32) {
    let mut char_iter = roll.chars();
    let mut values : HashMap<String, u32> = HashMap::new();
    loop {
        let count_text : String = char_iter.clone().take_while(|c| c.is_digit(10)).collect();
        advance_iter(&mut char_iter, count_text.len());
        let count : u32 = count_text.parse().expect("invalid count");
        char_iter.next();
        let color : String = char_iter.clone().take_while(|c| c.is_alphabetic()).collect();
        advance_iter(&mut char_iter, color.len());
        values.insert(color, count);
        if char_iter.next() == None {
            break;
        }
        else {
            char_iter.next();
        }
    }
    let red = values.get("red".into()).copied().unwrap_or(0);
    let green = values.get("green".into()).copied().unwrap_or(0);
    let blue = values.get("blue".into()).copied().unwrap_or(0);
    (red, green, blue)
}

fn validate_game(game : &Game) -> bool {
    const PREDICATE : (u32, u32, u32) = (12, 13, 14);
    game.counts.iter().map(|count| count.0 <= PREDICATE.0 && count.1 <= PREDICATE.1 && count.2 <= PREDICATE.2).fold(true, |a, b| a && b)
}

fn game_power(game : &Game) -> u32 {
    let red_max = game.counts.iter().map(|count| count.0).max().unwrap();
    let green_max = game.counts.iter().map(|count| count.1).max().unwrap();
    let blue_max = game.counts.iter().map(|count| count.2).max().unwrap();
    red_max * green_max * blue_max
}
