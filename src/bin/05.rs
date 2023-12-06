use itertools::Itertools;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let (seeds, maps) = input.split_once("\n\n").expect("expect valid input");

    let mut seeds: Vec<(u64, bool)> = seeds
        .strip_prefix("seeds: ")?
        .split_whitespace()
        .map(|n| (n.parse().expect("should be a number"), false))
        .collect();

    maps.split("\n\n").for_each(|map| {
        map.lines().skip(1).for_each(|line| {
            let numbers: Vec<_> = line
                .split_whitespace()
                .map(|n| n.parse::<u64>().expect("should be a number"))
                .collect();
            let source = numbers[1];
            let destination = numbers[0];
            let length = numbers[2];
            let end = source + length;
            for (seed, modified) in &mut seeds {
                if !*modified && *seed >= source && *seed < end {
                    *seed = destination + *seed - source;
                    *modified = true;
                }
            }
        });

        for (_, modified) in &mut seeds {
            *modified = false;
        }
    });

    seeds
        .iter()
        .map(|(seed, _)| seed)
        .min()
        .map(|num| num.to_owned())
}

trait InclusiveRangeExt
where
    Self: Sized,
{
    type Index;

    fn overlaps(&self, other: &Self) -> bool;
    fn get_overlap(&self, other: &Self) -> Option<Self>;
    fn get_rest(&self, other: &Self) -> Vec<Self>;
    fn offset(&self, offset: Self::Index) -> Self;
}

use num_traits::int::PrimInt;
use std::{ops::RangeInclusive, vec};

impl<T> InclusiveRangeExt for RangeInclusive<T>
where
    T: Ord + PrimInt + Copy,
{
    type Index = T;
    fn overlaps(&self, other: &Self) -> bool {
        other.end() >= self.start() && self.end() >= other.start()
    }

    fn get_overlap(&self, other: &Self) -> Option<Self> {
        if !self.overlaps(other) {
            return None;
        }

        let start = self.start().max(other.start()).min(other.end()).to_owned();
        let end = self.end().max(other.start()).min(other.end()).to_owned();
        Some(start..=end)
    }

    fn get_rest(&self, other: &Self) -> Vec<Self> {
        if !self.overlaps(other) {
            return vec![self.to_owned()];
        }

        let mut result = vec![];

        if self.start() >= other.start() && self.end() <= other.end() {
            return result;
        }

        if self.contains(other.start()) {
            result.push(*self.start()..=(*other.start() - T::one()));
        }

        if self.contains(other.end()) {
            result.push((*other.end() + T::one())..=*self.end());
        }

        result
    }

    fn offset(&self, offset: T) -> Self {
        (*self.start() + offset)..=(*self.end() + offset)
    }
}

type RangeMap = Vec<(Vec<RangeInclusive<i64>>, Vec<RangeInclusive<i64>>)>;

pub fn part_two(input: &str) -> Option<i64> {
    let (seeds, maps) = input.split_once("\n\n").expect("expect valid input");

    let mut seeds: RangeMap = seeds
        .strip_prefix("seeds: ")?
        .split_whitespace()
        .map(|n| n.parse().expect("should be a number"))
        .tuples()
        .map(|(start, length)| (vec![start..=(start + length - 1)], vec![]))
        .collect();

    maps.split("\n\n").for_each(|map| {
        map.lines().skip(1).for_each(|line| {
            let numbers: Vec<_> = line
                .split_whitespace()
                .map(|n| n.parse::<i64>().expect("should be a number"))
                .collect();
            let source = numbers[1];
            let end = numbers[1] + numbers[2] - 1;
            let offset = numbers[0] - numbers[1];

            let source_range = source..=end;

            for (from, to) in seeds.iter_mut() {
                *from = from
                    .iter()
                    .flat_map(|seed_range| {
                        if seed_range.overlaps(&source_range) {
                            let overlap = seed_range
                                .get_overlap(&source_range)
                                .unwrap()
                                .offset(offset);
                            let rest = seed_range.get_rest(&source_range);
                            to.push(overlap);
                            rest
                        } else {
                            vec![seed_range.clone()]
                        }
                    })
                    .collect();
            }
        });

        for (from, to) in seeds.iter_mut() {
            from.append(to);
            *to = vec![];
        }

        seeds = seeds
            .iter()
            .flat_map(|(from, to)| from.iter().chain(to))
            .map(|x| (vec![x.to_owned()], vec![]))
            .collect();
    });

    seeds
        .iter()
        .map(|(seed, _)| seed[0].start())
        .min()
        .map(|num| num.to_owned())
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[rstest]
    #[case(50..=55, 50..=55, true)] // equal
    #[case(50..=55, 53..=56, true)] // overlap after
    #[case(50..=55, 52..=54, true)] // within
    #[case(50..=55, 48..=51, true)] // overlap before
    #[case(50..=55, 48..=56, true)] // around
    #[case(50..=55, 32..=46, false)] // outside before
    #[case(50..=55, 72..=78, false)] // outside after
    fn test_ranges(
        #[case] first: RangeInclusive<u32>,
        #[case] second: RangeInclusive<u32>,
        #[case] expected: bool,
    ) {
        assert_eq!(first.overlaps(&second), expected);
    }

    #[rstest]
    #[case(50..=55, 50..=55, Some(50..=55))] // equal
    #[case(50..=55, 53..=56, Some(53..=55))] // overlap after
    #[case(50..=55, 52..=54, Some(52..=54))] // within
    #[case(50..=55, 48..=51, Some(50..=51))] // overlap before
    #[case(50..=55, 48..=56, Some(50..=55))] // around
    #[case(50..=55, 32..=46, None)] // outside before
    #[case(50..=55, 72..=78, None)] // outside after
    fn test_get_overlap(
        #[case] first: RangeInclusive<u32>,
        #[case] second: RangeInclusive<u32>,
        #[case] expected: Option<RangeInclusive<u32>>,
    ) {
        assert_eq!(first.get_overlap(&second), expected);
    }

    #[rstest]
    #[case(50..=55, 50..=55, vec![])] // equal
    #[case(50..=55, 53..=56, vec![50..=52])] // overlap after
    #[case(50..=57, 53..=55, vec![50..=52, 56..=57])] // within
    #[case(50..=57, 48..=51, vec![52..=57])] // overlap before
    #[case(50..=55, 48..=56, vec![])] // around
    #[case(50..=55, 32..=46, vec![50..=55])] // outside before
    #[case(50..=55, 72..=78, vec![50..=55])] // outside after
    fn test_get_rest(
        #[case] first: RangeInclusive<u32>,
        #[case] second: RangeInclusive<u32>,
        #[case] expected: Vec<RangeInclusive<u32>>,
    ) {
        assert_eq!(first.get_rest(&second), expected);
    }
}
