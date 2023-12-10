use std::env;
use std::fs;

fn main() {
    let filepath = env::args().nth(1).unwrap();
    let file_content = fs::read_to_string(filepath).unwrap();
    let (maze, startpos) = parse_maze(&file_content);
    let result = get_furthest_tile_distance(&maze, startpos);
    dbg!(result);
}

#[derive(Debug, Copy, Clone)]
enum Pipe {
    Vertical,
    Horizontal,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
}

fn parse_maze(file_content: &String) -> (Vec<Vec<Pipe>>, (usize, usize)) {
    let mut startpos: (usize, usize) = (0, 0);
    (
        file_content
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, character)| match character {
                        '|' => Pipe::Vertical,
                        '-' => Pipe::Horizontal,
                        'L' => Pipe::NE,
                        'J' => Pipe::NW,
                        '7' => Pipe::SW,
                        'F' => Pipe::SE,
                        '.' => Pipe::Ground,
                        'S' => {
                            startpos = (y, x);
                            Pipe::Start
                        }
                        _ => panic!(),
                    })
                    .collect()
            })
            .collect(),
        startpos,
    )
}

impl Pipe {
    fn connects_left(&self) -> bool {
        match self {
            Pipe::Horizontal => true,
            Pipe::NE => true,
            Pipe::SE => true,
            _ => false,
        }
    }

    fn connects_right(&self) -> bool {
        match self {
            Pipe::Horizontal => true,
            Pipe::NW => true,
            Pipe::SW => true,
            _ => false,
        }
    }

    fn connects_up(&self) -> bool {
        match self {
            Pipe::Vertical => true,
            Pipe::SE => true,
            Pipe::SW => true,
            _ => false,
        }
    }

    fn connects_down(&self) -> bool {
        match self {
            Pipe::Vertical => true,
            Pipe::NE => true,
            Pipe::NW => true,
            _ => false,
        }
    }
}

fn get_furthest_tile_distance(maze: &Vec<Vec<Pipe>>, startpos: (usize, usize)) -> u32 {
    //get connecting pipe to start
    let mut prev_pos = startpos;
    let mut pos = get_connecting_tile(maze, startpos);
    let mut counter: u32 = 0;
    //go through maze until you get back to start
    loop {
        let curr = maze[pos.0][pos.1];
        match curr {
            Pipe::Vertical => match prev_pos.0 < pos.0 {
                true => {
                    prev_pos = pos;
                    pos = (pos.0 + 1, pos.1);
                }
                false => {
                    prev_pos = pos;
                    pos = (pos.0 - 1, pos.1);
                }
            },
            Pipe::Horizontal => match prev_pos.1 < pos.1 {
                true => {
                    prev_pos = pos;
                    pos = (pos.0, pos.1 + 1);
                }
                false => {
                    prev_pos = pos;
                    pos = (pos.0, pos.1 - 1);
                }
            },
            Pipe::NE => match prev_pos.0 == pos.0 {
                true => {
                    prev_pos = pos;
                    pos = (pos.0 - 1, pos.1);
                }
                false => {
                    prev_pos = pos;
                    pos = (pos.0, pos.1 + 1);
                }
            },
            Pipe::NW => match prev_pos.0 == pos.0 {
                true => {
                    prev_pos = pos;
                    pos = (pos.0 - 1, pos.1);
                }
                false => {
                    prev_pos = pos;
                    pos = (pos.0, pos.1 - 1);
                }
            },
            Pipe::SE => match prev_pos.0 == pos.0 {
                true => {
                    prev_pos = pos;
                    pos = (pos.0 + 1, pos.1);
                }
                false => {
                    prev_pos = pos;
                    pos = (pos.0, pos.1 + 1);
                }
            },
            Pipe::SW => match prev_pos.0 == pos.0 {
                true => {
                    prev_pos = pos;
                    pos = (pos.0 + 1, pos.1);
                }
                false => {
                    prev_pos = pos;
                    pos = (pos.0, pos.1 - 1);
                }
            },
            Pipe::Start => break,
            Pipe::Ground => panic!(),
        }
        counter += 1;
    }
    //divide count by 2 and add 1
    counter / 2 + 1
}

fn get_connecting_tile(maze: &Vec<Vec<Pipe>>, pos: (usize, usize)) -> (usize, usize) {
    let left = (pos.0, pos.1 - 1);
    let right = (pos.0, pos.1 + 1);
    let up = (pos.0 - 1, pos.1);
    let down = (pos.0 + 1, pos.1);
    if maze[left.0][left.1].connects_left() {
        return left;
    };
    if maze[right.0][right.1].connects_right() {
        return right;
    };
    if maze[up.0][up.1].connects_up() {
        return up;
    };
    if maze[down.0][down.1].connects_down() {
        return down;
    };
    pos
}
