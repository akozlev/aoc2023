advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(|line| line.trim_matches(|c: char| c.is_ascii_lowercase()))
        .map(|line| {
            let mut str = String::new();
            str.push(line.chars().next()?);
            str.push(line.chars().last()?);
            str.parse::<u32>().ok()
        })
        .sum();
    result
}

const NUMBERS: [(&str, u32); 18] = [
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
    ("one", 1),
    ("two", 2),
    ("six", 6),
    ("four", 4),
    ("five", 5),
    ("nine", 9),
    ("three", 3),
    ("seven", 7),
    ("eight", 8),
];

use aho_corasick::AhoCorasick;

pub fn part_two(input: &str) -> Option<u32> {
    let ac = AhoCorasick::new(NUMBERS.map(|n| n.0)).unwrap();
    let acr = AhoCorasick::new(NUMBERS.map(|n| n.0.chars().rev().collect::<String>())).unwrap();
    let result = input
        .lines()
        .map(|line| {
            let rev_line = line.chars().rev().collect::<String>();
            let mat = ac.find(line).expect("Should have a number");
            let first = NUMBERS
                .iter()
                .find(|pair| pair.0 == &line[mat.start()..mat.end()])
                .unwrap();
            let mat = acr.find(&rev_line).expect("Should have a number");
            let last = NUMBERS
                .iter()
                .find(|pair| {
                    pair.0
                        == rev_line[mat.start()..mat.end()]
                            .chars()
                            .rev()
                            .collect::<String>()
                })
                .unwrap();
            first.1 * 10 + last.1
        })
        .sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
