use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let filepath = env::args().nth(1).unwrap();
    let file_content = fs::read_to_string(filepath).unwrap();
    let mut lines = file_content.lines();
    let documents = Documents::new(&mut lines);
    let result: usize = documents.count();
    dbg!(result);
}

#[derive(Debug)]
enum Instruction {
    Left,
    Right,
}

fn instruction_list_from_line(line: &str) -> Vec<Instruction> {
    line.chars()
        .map(|c| match c {
            'R' => Instruction::Right,
            'L' => Instruction::Left,
            _ => panic!("invalid instruction"),
        })
        .collect()
}

#[derive(Debug)]
struct Documents {
    instructions: Vec<Instruction>,
    path: HashMap<String, (String, String)>,
    current: String,
    cursor: usize,
}

impl<'a> Documents {
    fn new<T: Iterator<Item = &'a str>>(lines: &mut T) -> Documents {
        let instructions = instruction_list_from_line(lines.next().unwrap());
        lines.next();
        let mut path: HashMap<String, (String, String)> = HashMap::new();
        let re = regex::Regex::new(r"(\w{3}) = \((\w{3}), (\w{3})\)").unwrap();
        for line in lines {
            let captures = re.captures(line).unwrap();
            path.insert(
                captures[1].to_string(),
                (captures[2].to_string(), captures[3].to_string()),
            );
        }
        Documents {
            instructions,
            path,
            current: String::from("AAA"),
            cursor: 0,
        }
    }
}

impl Iterator for Documents {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == "ZZZ" {
            return None;
        }
        let instruction = &self.instructions[self.cursor];
        self.cursor = (self.cursor + 1) % self.instructions.len();
        let options = self.path.get(&self.current);
        let next = match instruction {
            Instruction::Left => &options.unwrap().0,
            Instruction::Right => &options.unwrap().1,
        };
        self.current = next.into();
        Some(1)
    }
}
