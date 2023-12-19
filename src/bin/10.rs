use std::collections::{HashSet, VecDeque};

// use colored::Colorize;

advent_of_code::solution!(10);

struct Dir {
    x: i16,
    y: i16,
    offest_a: usize,
    offest_b: usize,
}

//NESW
const DIRS: [Dir; 4] = [
    // North
    Dir {
        x: 0,
        y: -1,
        offest_a: 3,
        offest_b: 1,
    },
    // East
    Dir {
        x: 1,
        y: 0,
        offest_a: 2,
        offest_b: 0,
    },
    // South
    Dir {
        x: 0,
        y: 1,
        offest_a: 1,
        offest_b: 3,
    },
    // West
    Dir {
        x: -1,
        y: 0,
        offest_a: 0,
        offest_b: 2,
    },
];

fn adjacency(pipe: &char) -> u16 {
    match pipe {
        'S' => 0b1111,
        '.' => 0b0000,
        '|' => 0b1010,
        '-' => 0b0101,
        'L' => 0b1100,
        'F' => 0b0110,
        '7' => 0b0011,
        'J' => 0b1001,
        _ => panic!("Unknown pipe"),
    }
}

fn is_adjacent(a: &char, b: &char, dir: &Dir) -> bool {
    (adjacency(a) & (1 << dir.offest_a)) >> dir.offest_a == 1
        && (adjacency(b) & (1 << dir.offest_b)) >> dir.offest_b == 1
}

fn in_bounds(x: i16, y: i16, width: i16, height: i16) -> bool {
    let range_x = 0..width;
    let range_y = 0..height;
    range_x.contains(&x) && range_y.contains(&y)
}

#[derive(Debug)]
struct Node {
    x: i16,
    y: i16,
    dist: u32,
    pipe: char,
}

pub fn part_one(input: &str) -> Option<u32> {
    let width = input.lines().next().unwrap().len() as i16;
    let height = input.lines().count() as i16;
    let mut start: (i16, i16) = (0, 0);
    let pipes: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    'outer: for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                start = (x as i16, y as i16);
                break 'outer;
            }
        }
    }

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut max = 0;

    queue.push_back(Node {
        x: start.0,
        y: start.1,
        dist: 0,
        pipe: 'S',
    });

    while !queue.is_empty() {
        let current: Node = queue.pop_front().unwrap();
        max = max.max(current.dist);
        visited.insert((current.x, current.y));

        for dir in DIRS {
            let coord = (current.x + dir.x, current.y + dir.y);
            let (x, y) = coord;

            if visited.contains(&coord) || !in_bounds(x, y, width, height) {
                continue;
            }

            let b = pipes.get(y as usize)?.get(x as usize).unwrap();
            if is_adjacent(&current.pipe, b, &dir) {
                queue.push_back(Node {
                    x,
                    y,
                    dist: current.dist + 1,
                    pipe: b.to_owned(),
                });
            }
        }
    }

    Some(max)
}

#[derive(Debug)]
struct Node2 {
    pub x: i16,
    pub y: i16,
    pub pipe: char,
}

pub fn part_two(input: &str) -> Option<u32> {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let mut start: (i16, i16) = (0, 0);
    let pipes: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    'outer: for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                start = (x as i16, y as i16);
                break 'outer;
            }
        }
    }

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back(Node2 {
        x: start.0,
        y: start.1,
        pipe: 'S',
    });

    while !queue.is_empty() {
        let current: Node2 = queue.pop_front().unwrap();
        visited.insert((current.x, current.y));

        for dir in DIRS {
            let coord = (current.x + dir.x, current.y + dir.y);
            let (x, y) = coord;

            if visited.contains(&coord) || !in_bounds(x, y, width as i16, height as i16) {
                continue;
            }

            let b = pipes.get(y as usize)?.get(x as usize).unwrap();
            if is_adjacent(&current.pipe, b, &dir) {
                queue.push_back(Node2 { x, y, pipe: *b });
            }
        }
    }

    let mut edges: u16;
    let mut enclosed = 0u32;

    for (y, row) in pipes.iter().enumerate() {
        for x in 0..row.len() {
            let current = (x as i16, y as i16);
            if visited.contains(&current) {
                // print!("{}", cell.to_string().blue());
                continue;
            }
            edges = 0;

            let mut x2 = x;
            let mut y2 = y;

            while x2 < width && y2 < height {
                let current = (x2 as i16, y2 as i16);
                let pipe = pipes.get(y2).unwrap().get(x2).unwrap();
                if visited.contains(&current) && pipe != &'L' && pipe != &'7' {
                    edges += 1;
                }

                x2 += 1;
                y2 += 1;
            }
            if edges % 2 == 1 {
                enclosed += 1;
                // print!("{}", cell.to_string().red());
            } else {
                // print!("{}", cell);
            }
        }
        // print!(" {}", edges);
        // println!();
    }

    Some(enclosed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
