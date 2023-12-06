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
}

#[derive(Debug)]
struct Race {
    time: u32,
    distance: u32,
}

impl Race {
    fn win_count(&self) -> u32 {
        (0..=self.time)
            .filter(|press| press * (self.time - press) > self.distance)
            .count() as u32
    }
}
