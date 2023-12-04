use std::collections::HashMap;

advent_of_code::solution!(3);

fn is_symbol(ch: char) -> bool {
    !ch.is_ascii_digit() && ch != '.'
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<_> = input.lines().collect();
    let max_y = grid.len() - 1;
    let max_x = grid[0].len() - 1;

    let mut sum = 0u32;

    for (y, &line) in grid.iter().enumerate() {
        let mut number: Option<u32> = None;
        let mut start = 0usize;

        for (x, ch) in line.chars().enumerate() {
            let mut check_num = false;
            if let Some(digit) = ch.to_digit(10) {
                if let Some(n) = number {
                    number = Some(n * 10 + digit);
                } else {
                    start = x;
                    number = Some(digit);
                }

                check_num = x == max_x;
            } else {
                check_num = number.is_some();
            }

            if check_num {
                let n = number.unwrap();
                // right
                let mut has_symbol = is_symbol(ch);
                let mut s = start;

                if start > 0 {
                    s -= 1;
                    // left
                    has_symbol |= line.chars().nth(s).is_some_and(is_symbol);
                }

                // above
                if y > 0 {
                    has_symbol |= grid
                        .get(y - 1)
                        .map(|&line| line[s..=x].chars().any(is_symbol))
                        .unwrap_or(false);
                }
                // below
                if y < max_y {
                    has_symbol |= grid
                        .get(y + 1)
                        .map(|line| line[s..=x].chars().any(is_symbol))
                        .unwrap_or(false);
                }

                if has_symbol {
                    sum += n;
                }
                number = None;
            }
        }
    }

    Some(sum)
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Coord {
    x: usize,
    y: usize,
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid: Vec<_> = input.lines().collect();
    let max_y = grid.len() - 1;
    let max_x = grid[0].len() - 1;
    let mut map: HashMap<Coord, u64> = HashMap::new();

    let mut sum = 0u64;

    for (y, &line) in grid.iter().enumerate() {
        let mut number: Option<u64> = None;
        let mut start = 0usize;

        for (x, ch) in line.chars().enumerate() {
            let mut check_num = false;
            if let Some(digit) = ch.to_digit(10) {
                if let Some(n) = number {
                    number = Some(n * 10 + digit as u64);
                } else {
                    start = x;
                    number = Some(digit as u64);
                }

                check_num |= x == max_x;
            } else {
                check_num = number.is_some();
            }

            if check_num {
                let n = number.unwrap();
                let mut indices: Vec<Coord> = vec![];
                // construct array of indices
                indices.push(Coord { x, y });

                let mut s = start;

                if start > 0 {
                    s -= 1;
                    indices.push(Coord { x: s, y });
                }

                if y > 0 {
                    (s..=x).for_each(|i| indices.push(Coord { x: i, y: y - 1 }));
                }

                if y < max_y {
                    (s..=x).for_each(|i| indices.push(Coord { x: i, y: y + 1 }));
                }

                // indices.find() to find star
                if let Some(coord) = indices.iter().find(|coord| {
                    grid[coord.y]
                        .chars()
                        .nth(coord.x)
                        .map(|c| c == '*')
                        .unwrap_or(false)
                }) {
                    match map.get(coord) {
                        Some(num) => {
                            sum += num * n;
                        }
                        None => {
                            map.insert(coord.clone(), n);
                        }
                    };
                }
                number = None;
            }
        }
    }

    Some(sum)
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
