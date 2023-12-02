pub fn solve(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| {
            let mut parts = line.split(": ");

            let game = parts
                .next()
                .unwrap()
                .split_whitespace()
                .last()
                .unwrap()
                .parse::<u32>()
                .unwrap();

            let mut rounds = parts.next().unwrap().split("; ");

            let is_valid = rounds.all(|round| {
                round.split(", ").all(|cube| {
                    if let Some((value, color)) = cube.split_once(' ') {
                        let value = value.parse::<i32>().unwrap_or_default();

                        match color {
                            "red" => value <= 12,
                            "green" => value <= 13,
                            "blue" => value <= 14,
                            _ => false,
                        }
                    } else {
                        false
                    }
                })
            });

            if is_valid {
                return Some(game);
            }

            None
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_riddle() {
        let input = include_str!("test_input1.txt");
        assert_eq!(solve(input), 8);
    }
}
