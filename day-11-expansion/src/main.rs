use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;
use std::fs;

fn main() {
    let filepath = env::args().nth(1).unwrap();
    let file_content = fs::read_to_string(filepath).unwrap();
    let space = parse_space(&file_content);
    let expanded = expand_space(&space);
    let galaxies = get_galaxies(&expanded);
    let distances = calc_galaxies_distances(&expanded, &galaxies);
    let result : u32 = distances.iter().sum();
    dbg!(result);
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Space {
    Empty,
    Galaxy,
}

fn parse_space(file: &str) -> Vec<Vec<Space>> {
    file.lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Space::Empty,
                    '#' => Space::Galaxy,
                    _ => panic!("invalid character"),
                })
                .collect()
        })
        .collect()
}

fn expand_space(space: &Vec<Vec<Space>>) -> Vec<Vec<Space>> {
    // Get empty columns
    let mut empty_columns: HashSet<usize> = HashSet::new();
    for i in 0..space[0].len() {
        let mut j: usize = 0;
        while j < space.len() {
            match space[j][i] {
                Space::Galaxy => break,
                Space::Empty => j += 1,
            }
        }
        if j == space.len() {
            empty_columns.insert(i);
        }
    }
    // Get empty lines
    let empty_lines: HashSet<usize> = space
        .iter()
        .enumerate()
        .filter(|(_index, line)| line.iter().all(|spot| *spot == Space::Empty))
        .map(|(index, _)| index)
        .collect();
    let mut result: Vec<Vec<Space>> = Vec::with_capacity(space.len() + &empty_lines.len());
    for (index, space_line) in space.iter().enumerate() {
        match empty_lines.contains(&index) {
            true => {
                let mut to_push = space_line.clone();
                for _i in 0..empty_columns.len() {
                    to_push.push(Space::Empty);
                }
                result.push(to_push.clone());
                result.push(to_push.clone());
            }
            false => {
                let mut expanded_line: Vec<Space> =
                    Vec::with_capacity(space[0].len() + &empty_columns.len());
                for (sub_index, space_tile) in space_line.iter().enumerate() {
                    match empty_columns.contains(&sub_index) {
                        true => {
                            expanded_line.push(space_tile.clone());
                            expanded_line.push(space_tile.clone());
                        }
                        false => expanded_line.push(space_tile.clone()),
                    };
                }
                result.push(expanded_line);
            }
        }
    }
    result
}

fn get_galaxies(space : &Vec<Vec<Space>>) -> Vec<(usize,usize)> {
    let mut result : Vec<(usize,usize)> = vec![];
    for (y, line) in space.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            match tile {
                Space::Galaxy => result.push((y,x)),
                _ => (),
            }
        }
    }
    result
}

fn calc_galaxies_distances(space : &Vec<Vec<Space>>, galaxies : &Vec<(usize,usize)>) -> Vec<u32> {
    let mut visited : HashSet<(usize,usize)> = HashSet::new();
    let mut result : Vec<u32> = Vec::with_capacity((galaxies.len() * galaxies.len())/2);
    for galaxy in galaxies.iter() {
        let mut distances  : Vec<u32> = calc_galaxy_distances(space, *galaxy, &mut visited.clone());
        result.append(&mut distances);
        visited.insert(*galaxy);
    };
    result
}

#[no_mangle]
fn calc_galaxy_distances(space : &Vec<Vec<Space>>, galaxy : (usize,usize), visited : &mut HashSet<(usize,usize)>) -> Vec<u32> {
    let mut distances : Vec<u32> = vec![];
    let mut queue : VecDeque<(usize, usize, u32)> = VecDeque::new();
    visited.insert(galaxy);
    queue.push_back((galaxy.0, galaxy.1, 0));
    while !queue.is_empty() {
        let (y,x,dist) = queue.pop_front().unwrap();
        match space[y][x] {
            Space::Galaxy => distances.push(dist),
            _ => (),
        }
        for tile in get_adjacent_tiles(space, (y,x), visited) {
            queue.push_back((tile.0, tile.1, dist + 1));
            visited.insert(tile);
        }
    }
    distances
}

fn get_adjacent_tiles(space : &Vec<Vec<Space>>, tile : (usize, usize), visited : &HashSet<(usize,usize)>) -> Vec<(usize,usize)> {
    let mut result : Vec<(usize,usize)> = vec![];
    if tile.0 > 0 {
        match !visited.contains(&(tile.0 - 1, tile.1)) {
            true => result.push((tile.0 - 1, tile.1)),
            false => (),
        }
    };
    if tile.0 < space.len() - 1 {
        match !visited.contains(&(tile.0 + 1, tile.1)) {
            true => result.push((tile.0 + 1, tile.1)),
            false => (),
        }
    };
    if tile.1 > 0 {
        match !visited.contains(&(tile.0, tile.1 - 1)) {
            true => result.push((tile.0, tile.1 - 1)),
            false => (),
        }
    };
    if tile.1 < space[0].len() - 1 {
        match !visited.contains(&(tile.0, tile.1 + 1)) {
            true => result.push((tile.0, tile.1 + 1)),
            false => (),
        }
    };
    result
}
