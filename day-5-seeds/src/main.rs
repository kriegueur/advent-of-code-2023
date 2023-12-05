use std::collections::VecDeque;
use std::env;
use std::fs;

#[derive(Debug)]
struct RangeMap {
    destination : u64,
    source : u64,
    length : u64,
}

fn main() {
    let filepath = env::args().nth(1).expect("expected filepath as first argument");
    let lines = read_lines(&filepath);
    let mut blocks : Vec<Vec<&String>> = vec![];
    blocks.reserve(8);
    let mut buffer_vec : Vec<&String> = vec![];
    for line in lines.iter() {
        if line.is_empty() {
            blocks.push(buffer_vec);
            buffer_vec = vec![];
        }
        else {
            buffer_vec.push(line);
        }
    };
    blocks.push(buffer_vec);
    let seeds = seeds_from_line(&blocks[0][0]);
    let mut range_seeds = unwrap_seeds(&seeds);
    let conversions : Vec<Vec<RangeMap>> = blocks.iter().skip(1).map(|block| maps_from_block(block)).collect();
    conversions.iter().for_each(|c| range_seeds = convert_ranges(&range_seeds, &c));
    let result = range_seeds.iter().map(|rs| rs.0).min().unwrap();
    println!("{}", result);
}

fn read_lines(filepath : &str) -> Vec<String> {
    fs::read_to_string(filepath).expect("invalid filepath").lines().map(String::from).collect()
}

fn seeds_from_line(line : &str) -> Vec<u64> {
    let numbers = line.split(": ").nth(1).unwrap();
    numbers.split(' ').map(|n| n.parse().unwrap()).collect()
}

fn maps_from_block(block : &Vec<&String>) -> Vec<RangeMap> {
    let mut lines_iter = block.iter();
    lines_iter.next();
    lines_iter.map(|line| map_from_line(line)).collect()
}

fn map_from_line(line : &str) -> RangeMap {
    let mut numbers = line.split(' ').map(|n| n.parse::<u64>().unwrap());
    let destination = numbers.next().unwrap();
    let source = numbers.next().unwrap();
    let length = numbers.next().unwrap();
    RangeMap { destination: destination, source: source, length: length }
}

fn convert(values : &Vec<u64>, maps : &Vec<RangeMap>) -> Vec<u64> {
    values.iter().map(|v| {for map in maps {
        if map.has(*v) {
            return map.convert(*v);
        }
    };
    *v}).collect()
}

fn convert_ranges(ranges : &Vec<(u64,u64)>, maps : &Vec<RangeMap>) -> Vec<(u64,u64)> {
    let mut work = ranges.iter().copied().collect::<VecDeque<(u64,u64)>>();
    let mut result : Vec<(u64,u64)> = vec![];
    while !work.is_empty() {
        let range = work.pop_front().unwrap();
        let mut found = false;
        for map in maps {
            if !found {
                if map.has_range(range) {
                    let (left, middle, right) = map.contained_range(range);
                    if left.1 != 0 { work.push_back(left); }
                    if right.1 != 0 { work.push_back(right); }
                    let converted = (map.convert(middle.0), middle.1);
                    result.push(converted);
                    found = true;
                }
            }
        };
        if !found {result.push(range)}
    };
    result
}

impl RangeMap {
    fn has(&self, value : u64) -> bool {
        value >= self.source && value < self.source + self.length
    }

    fn convert(&self, value : u64) -> u64 {
        if self.destination > self.source {
            self.destination - self.source + value
        }
        else {
            //self.destination + value - self.source
            value - self.source + self.destination
        }
    }

    fn has_range(&self, range : (u64,u64)) -> bool {
        let end = range.0 + range.1 - 1;
        (range.0 >= self.source && range.0 < self.length + self.source) ||
            (end >= self.source && end < self.length + self.source) ||
            (range.0 < self.source && end > self.source + self.length)
    }

    fn contained_range(&self, range : (u64, u64)) -> ((u64,u64),(u64,u64),(u64,u64)) {
        let end = range.0 + range.1 - 1;
        let contained_start = if range.0 > self.source {
            range.0
        } else {
            self.source
        };
        let contained_end = if end < self.source + self.length {
            end
        } else {
            self.source + self.length - 1
        };
        ((range.0, contained_start - range.0),(contained_start, contained_end - contained_start + 1),(contained_end + 1, end - contained_end))
    }
}

fn unwrap_seeds(seeds : &Vec<u64>) -> Vec<(u64,u64)> {
    let mut result : Vec<(u64,u64)> = vec![];
    let mut buffer : Option<u64> = None;
    for x in seeds {
        match buffer {
            Some(prev) => {
                result.push((prev, *x));
                buffer = None;
            },
            None => buffer = Some(*x),
        }
    };
    result
}
