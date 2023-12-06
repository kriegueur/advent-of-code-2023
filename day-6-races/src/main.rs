use std::env;
use std::fs;

fn main() {
    let filepath = env::args().nth(1).unwrap();
    let lines = fs::read_to_string(filepath)
        .expect("invalid filename")
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();
    let re = regex::Regex::new(r"\d+").unwrap();
    /* Part 1
    let times = re
        .find_iter(&lines[0])
        .map(|s| s.as_str().parse::<u32>().unwrap());
    let distances = re
        .find_iter(&lines[1])
        .map(|s| s.as_str().parse::<u32>().unwrap());
    let races = times.zip(distances).map(|(time, dist)| Race {
        time,
        distance: dist,
    });
    let counts = races.map(|race| race.win_count());
    let result: u32 = counts.product();
    dbg!(result);
    */
    let time: u64 = re
        .find_iter(&lines[0])
        .fold(String::new(), |s, found_match| s + found_match.as_str())
        .parse()
        .unwrap();
    let dist: String = re
        .find_iter(&lines[1])
        .fold(String::new(), |s, found_match| s + found_match.as_str());
    let race = Race {
        time,
        distance: dist.parse().unwrap(),
    };
    let result = race.win_count();
    dbg!(result);
}

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn win_count(&self) -> u64 {
        (0..=self.time)
            .filter(|press| press * (self.time - press) > self.distance)
            .count() as u64
    }
}
