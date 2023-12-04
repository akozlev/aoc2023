advent_of_code::solution!(2);

fn get_game_id(game_string: &str) -> Option<u32> {
    game_string.trim_start_matches("Game ").parse::<u32>().ok()
}

pub fn part_one(input: &str) -> Option<u32> {
    let max_cube = [("red", 12), ("green", 13), ("blue", 14)];
    let result = input
        .lines()
        .map(|line| {
            let split = line.split_once(": ")?;
            let game = get_game_id(split.0)?;
            let rounds_valid = split
                .1
                .split("; ")
                .map(|round| {
                    round
                        .split(", ")
                        .map(|cubes| {
                            let pair = cubes.trim().split_once(' ').expect("needs to be a pair");
                            let n_cubes = pair.0.parse::<u32>().expect("should be a number");
                            let cube_type = max_cube
                                .iter()
                                .find(|cube| cube.0 == pair.1)
                                .expect("should be a color");
                            n_cubes <= cube_type.1
                        })
                        .all(|x| x)
                })
                .all(|x| x);
            rounds_valid.then_some(game)
        })
        .sum();
    result
}

pub fn part_two(input: &str) -> Option<u32> {
    let colors: [&str; 3] = ["red", "green", "blue"];
    let result = input
        .lines()
        .map(|line| {
            let split = line.split_once(": ")?;
            let mut min_colors: [u32; 3] = [0, 0, 0];
            split.1.split("; ").for_each(|round| {
                round.split(", ").for_each(|cubes| {
                    let pair = cubes.trim().split_once(' ').expect("needs to be a pair");
                    let n_cubes = pair.0.parse::<u32>().expect("should be a number");
                    let cube_index = colors
                        .iter()
                        .position(|c| c == &pair.1)
                        .expect("should be a in vec");
                    min_colors[cube_index] = min_colors[cube_index].max(n_cubes);
                })
            });
            Some(min_colors[0] * min_colors[1] * min_colors[2])
        })
        .sum();
    result
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
