use std::collections::{HashMap, HashSet};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(|line| {
            let (_, numbers) = line
                .split_once(": ")
                .expect("should be prefix with \"Card N:\"");

            let (winning_numbers, drawn_numbers) =
                numbers.split_once(" | ").expect("should have separator");

            let winning: HashSet<_> = winning_numbers
                .split_whitespace()
                .map(|num| num.parse::<u32>().expect("should be a number"))
                .collect();

            let drawn: HashSet<_> = drawn_numbers
                .split_whitespace()
                .map(|num| num.parse::<u32>().expect("should be a number"))
                .collect();

            let common_count = winning.intersection(&drawn).count() as u32;

            if common_count == 0 {
                0
            } else {
                2u32.pow(common_count - 1u32)
            }
        })
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut games: HashMap<u32, u32> = HashMap::new();

    input.lines().for_each(|line| {
        let (game, numbers) = line
            .split_once(": ")
            .expect("should be prefix with \"Card N:\"");

        let game = game
            .split_whitespace()
            .last()
            .map(|n| n.parse::<u32>().expect("should be a number"))
            .expect("should be a number");

        let (winning_part, drawn_part) = numbers.split_once(" | ").expect("should have separator");

        let winning: HashSet<_> = winning_part
            .split_whitespace()
            .map(|num| num.parse::<u32>().expect("should be a number"))
            .collect();

        let drawn: HashSet<_> = drawn_part
            .split_whitespace()
            .map(|num| num.parse::<u32>().expect("should be a number"))
            .collect();

        let common_count = winning.intersection(&drawn).count() as u32;

        let instances = games
            .entry(game)
            .and_modify(|count| *count += 1)
            .or_insert(1)
            .to_owned();

        if common_count > 0 {
            for game_id in (game + 1)..=(game + common_count) {
                games
                    .entry(game_id)
                    .and_modify(|count| *count += instances)
                    .or_insert(instances);
            }
        }
    });

    Some(games.values().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
