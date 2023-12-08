use itertools::Itertools;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<f32> {
    let (time, distance) = input
        .split_once('\n')
        .expect("should have time and distance");

    let result = time
        .split_whitespace()
        .skip(1)
        .interleave(distance.split_whitespace().skip(1))
        .map(|num| num.parse::<f32>().expect("should be a number"))
        .tuples()
        .map(|(a, b)| {
            let half_a = a / 2.0;
            let discriminant = (a.powi(2) - 4.0 * b).sqrt() / 2.0;

            let solution1 = (half_a - discriminant).floor();
            let solution2 = (half_a + discriminant).ceil() - 1.0;

            Some(solution2 - solution1)
        })
        .product();

    result
}

pub fn part_two(input: &str) -> Option<f32> {
    let (time, distance) = input
        .split_once('\n')
        .expect("should have time and distance");

    let a = time
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<f32>()
        .expect("should be a number");

    let b = distance
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<f32>()
        .expect("should be a number");

    let half_a = a / 2.0;
    let discriminant = (a.powi(2) - 4.0 * b).sqrt() / 2.0;

    let solution1 = (half_a - discriminant).floor();
    let solution2 = (half_a + discriminant).ceil() - 1.0;

    Some(solution2 - solution1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288.0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503.0));
    }
}
