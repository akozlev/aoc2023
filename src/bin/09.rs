use std::collections::VecDeque;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i32> {
    let mut result = 0;
    input.lines().for_each(|line| {
        let mut nums: Vec<_> = line
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        while !nums.iter().all(|&x| x == 0) {
            for i in 0..(nums.len() - 1usize) {
                nums[i] = nums[i + 1] - nums[i];
            }
            result += nums.pop().unwrap();
        }
    });

    Some(result)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut result = 0;
    input.lines().for_each(|line| {
        let mut nums: VecDeque<_> = line
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        println!("{result} {:?}", nums);
        for i in (1..(nums.len())).rev() {
            nums[i] -= nums[i - 1];
        }
        println!("{result} {:?}", nums);
        result += nums.pop_front().unwrap();
        loop {
            for i in (1..(nums.len())).rev() {
                nums[i] -= nums[i - 1];
            }
            println!("{result} {:?}", nums);
            result -= nums.pop_front().unwrap();
            if nums.iter().all(|&x| x == 0) {
                break;
            }
        }
        println!();
    });

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
