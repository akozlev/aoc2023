use std::{collections::HashSet, ops::Range};

use colored::ColoredString;
use itertools::Itertools;

advent_of_code::solution!(11);

fn ranges(a: &(usize, usize), b: &(usize, usize)) -> (Range<usize>, Range<usize>) {
    let x = if a.0 < b.0 { a.0..b.0 } else { b.0..a.0 };
    let y = if a.1 < b.1 { a.1..b.1 } else { b.1..a.1 };
    (x, y)
}

fn _debug_space<P>(universe: &str, predicate: P)
where
    P: Fn(usize, usize, char) -> ColoredString,
{
    for (y, row) in universe.lines().enumerate() {
        print!("{y:>3}: ");
        for (x, cell) in row.chars().enumerate() {
            print!(
                "{}",
                predicate(x, y, cell));
        }
        println!();
    }
    println!();
    println!();
}

pub fn part_one(input: &str) -> Option<u32> {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    let mut cols: HashSet<_> = (0..width).collect();
    let mut rows: HashSet<_> = (0..height).collect();
    let mut galaxies = HashSet::new();

    for (y, row) in input.lines().enumerate() {
        for (x, cell) in row.chars().enumerate() {
            if cell == '#' {
                rows.remove(&y);
                cols.remove(&x);
                galaxies.insert((x, y));
            }
        }
    }

    let result = galaxies
        .iter()
        .tuple_combinations()
        .map(|(a, b)| {
            let (rx, ry) = ranges(a, b);
            let col = cols.iter().filter(|x| rx.contains(x)).count() as i32;
            let row = rows.iter().filter(|y| ry.contains(y)).count() as i32;

            let x1 = a.0 as i32;
            let y1 = a.1 as i32;
            let x2 = b.0 as i32;
            let y2 = b.1 as i32;

            let c = (y2 - y1).abs() + col + (x2 - x1).abs() + row;

            return c as u32;
        })
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    let mut cols: HashSet<_> = (0..width).collect();
    let mut rows: HashSet<_> = (0..height).collect();
    let mut galaxies = HashSet::new();

    for (y, row) in input.lines().enumerate() {
        for (x, cell) in row.chars().enumerate() {
            if cell == '#' {
                rows.remove(&y);
                cols.remove(&x);
                galaxies.insert((x, y));
            }
        }
    }

    let result = galaxies
        .iter()
        .tuple_combinations()
        .map(|(a, b)| {
            let (rx, ry) = ranges(a, b);
            let col = cols.iter().filter(|x| rx.contains(x)).count() as i32 * 999_999;
            let row = rows.iter().filter(|y| ry.contains(y)).count() as i32 * 999_999;

            let x1 = a.0 as i32;
            let y1 = a.1 as i32;
            let x2 = b.0 as i32;
            let y2 = b.1 as i32;

            let c = (y2 - y1).abs() + col + (x2 - x1).abs() + row;

            return c as u64;
        })
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
