use num::integer::lcm;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let filepath = env::args().nth(1).unwrap();
    let file_content = fs::read_to_string(filepath).unwrap();
    let mut lines = file_content.lines();
    let (instructions, documents) = parse_lines(&mut lines);
    let starts = documents
        .keys()
        .filter(|k| k.chars().last().unwrap() == 'A')
        .collect::<Vec<&String>>();
    let mut paths: Vec<_> = starts
        .iter()
        .map(|s| Path::new(&instructions, &documents, s))
        .collect();
    let mut ranges: Vec<Range> = vec![];
    for i in 0..paths.len() {
        let mut path = &mut paths[i];
        ranges.push(find_path_loop(&mut path));
    }
    let lcm: u64 = ranges
        .iter()
        .map(|r| r.get() as u64)
        .fold(1, |a, b| lcm(a, b));
    dbg!(lcm);
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

fn parse_lines<'a, T: Iterator<Item = &'a str>>(
    lines: &mut T,
) -> (Vec<Instruction>, HashMap<String, (String, String)>) {
    let instructions = instruction_list_from_line(lines.next().unwrap());
    lines.next();
    let mut documents: HashMap<String, (String, String)> = HashMap::new();
    let re = regex::Regex::new(r"(\w{3}) = \((\w{3}), (\w{3})\)").unwrap();
    for line in lines {
        let captures = re.captures(line).unwrap();
        documents.insert(
            captures[1].to_string(),
            (captures[2].to_string(), captures[3].to_string()),
        );
    }
    (instructions, documents)
}

#[derive(Debug)]
struct Path<'a> {
    instructions: &'a Vec<Instruction>,
    documents: &'a HashMap<String, (String, String)>,
    current: &'a String,
    cursor: usize,
}

impl<'a> Path<'a> {
    fn new(
        instructions: &'a Vec<Instruction>,
        documents: &'a HashMap<String, (String, String)>,
        start: &'a String,
    ) -> Path<'a> {
        Path {
            instructions,
            documents,
            current: start,
            cursor: 0,
        }
    }
}

impl<'a> Iterator for Path<'a> {
    type Item = &'a String;

    fn next(&mut self) -> Option<Self::Item> {
        let instruction = &self.instructions[self.cursor];
        self.cursor = (self.cursor + 1) % self.instructions.len();
        let options = self.documents.get(self.current);
        let next = match instruction {
            Instruction::Left => &options.unwrap().0,
            Instruction::Right => &options.unwrap().1,
        };
        self.current = next;
        Some(self.current)
    }
}

fn find_path_loop(path: &mut Path) -> Range {
    let first = path.current;
    let mut positions: HashSet<(usize, &String)> = HashSet::new();
    let (start, startpos) = loop {
        let curr: &String = path.current;
        let pos: usize = path.cursor;
        if positions.contains(&(pos, curr)) {
            break (pos, curr);
        }
        if curr.chars().last().unwrap() == 'Z' {
            positions.insert((pos, curr));
        }
        path.next();
    };
    let mut loopstart: usize = 0;
    path.current = first;
    path.cursor = 0;
    loop {
        if path.cursor == start && path.current == startpos {
            break;
        }
        loopstart += 1;
        path.next();
    }
    let mut loop_length = 1;
    let mut jumps: Vec<usize> = vec![];
    path.next();
    while path.cursor != start || path.current != startpos {
        if path.current.chars().last().unwrap() == 'Z' {
            jumps.push(loop_length);
            loop_length = 0;
        } else {
            loop_length += 1;
        }
        path.next();
    }
    jumps.push(loop_length);
    dbg!(loopstart, &jumps);
    Range {
        pos: loopstart,
        jumps,
        cursor: 0,
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Range {
    pos: usize,
    jumps: Vec<usize>,
    cursor: usize,
}

impl Range {
    fn get(&self) -> usize {
        self.pos
    }
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get().cmp(&other.get())
    }
}
