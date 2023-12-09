use std::env;
use std::fs;

fn main() {
    let filepath = env::args().nth(1).unwrap();
    let file_content = fs::read_to_string(filepath).unwrap();
    let histories: Vec<Vec<i32>> = file_content
        .lines()
        .map(|l| l.split(' ').map(|d| d.parse().unwrap()).collect())
        .collect();
    /* part 1
    let result: i32 = histories.iter().map(calculate_next_value).sum();
    */
    let result: i32 = histories.iter().map(calculate_prev_value).sum();
    dbg!(result);
}

/* part 1
fn calculate_next_value(history: &Vec<i32>) -> i32 {
    if history.iter().all(|value| *value == 0) {
        return 0;
    };
    let mut next: Vec<i32> = Vec::with_capacity(history.len() - 1);
    for (prev, curr) in history.iter().zip(history.iter().skip(1)) {
        next.push(curr - prev);
    }
    let after = calculate_next_value(&next);
    history.last().unwrap() + after
}
*/

fn calculate_prev_value(history: &Vec<i32>) -> i32 {
    if history.iter().all(|value| *value == 0) {
        return 0;
    };
    let mut next: Vec<i32> = Vec::with_capacity(history.len() - 1);
    for (prev, curr) in history.iter().zip(history.iter().skip(1)) {
        next.push(curr - prev);
    }
    let before = calculate_prev_value(&next);
    history.first().unwrap() - before
}
