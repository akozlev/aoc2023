use std::collections::HashMap;
advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let paren: &[char] = &['(', ')'];
    let (moves, maps) = input.split_once("\n\n").unwrap();

    let mut network: HashMap<&str, (&str, &str)> = HashMap::new();
    maps.lines().for_each(|line| {
        let (key, pair) = line.split_once(" = ").unwrap();
        let value = pair.trim_matches(paren).split_once(", ").unwrap();
        network.insert(key, value);
    });

    let mut move_count = 0u32;
    let mut current = "AAA";

    let mut iter = moves.chars();

    while current != "ZZZ" {
        if let Some(m) = iter.next() {
            let node = network.get(current).unwrap();
            match m {
                'L' => current = node.0,
                'R' => current = node.1,
                _ => panic!("invalid move"),
            }

            move_count += 1;
        } else {
            iter = moves.chars();
        }
    }

    Some(move_count)
}

fn gcd(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;
    let mut r;
    while a % b > 0 {
        r = a % b;
        a = b;
        b = r;
    }
    b
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

pub fn part_two(input: &str) -> Option<u64> {
    let paren: &[char] = &['(', ')'];
    let (moves, maps) = input.split_once("\n\n").unwrap();

    let mut network: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut current: Vec<(&str, u64)> = vec![];
    maps.lines().for_each(|line| {
        let (key, pair) = line.split_once(" = ").unwrap();
        let value = pair.trim_matches(paren).split_once(", ").unwrap();
        network.insert(key, value);
        if let Some('A') = key.chars().nth(2) {
            current.push((key, 0));
        }
    });

    let mut iter;

    for (key, count) in current.iter_mut() {
        iter = moves.chars();
        while key.chars().nth(2).unwrap() != 'Z' {
            if let Some(m) = iter.next() {
                let node = network.get(key).unwrap();
                match m {
                    'L' => *key = node.0,
                    'R' => *key = node.1,
                    _ => panic!("invalid move"),
                }
                *count += 1;
            } else {
                iter = moves.chars();
            }
        }
    }

    let result = current.iter().fold(1, |acc, (_, count)| lcm(*count, acc));

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
