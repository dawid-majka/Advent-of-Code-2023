use std::collections::HashMap;

pub fn solve(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut cube_map: HashMap<&str, u32> = HashMap::new();

            let (_, rounds) = line.split_once(": ").unwrap_or_default();

            rounds.split("; ").for_each(|round| {
                round.split(", ").for_each(|cube| {
                    if let Some((value, color)) = cube.split_once(' ') {
                        let value = value.parse::<u32>().unwrap_or_default();

                        if cube_map.contains_key(color) {
                            if cube_map.get(color).unwrap() < &value {
                                cube_map.insert(color, value);
                            }
                        } else {
                            cube_map.insert(color, value);
                        }
                    }
                })
            });
            cube_map.values().product::<u32>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_riddle() {
        let input = include_str!("test_input1.txt");
        assert_eq!(solve(input), 2286);
    }
}
