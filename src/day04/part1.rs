use std::collections::HashSet;

pub fn solve(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| {
            if let Some((_, numbers)) = line.split_once(": ") {
                if let Some((winning, my)) = numbers.split_once(" |") {
                    let winning_nums: HashSet<&str> = winning.split_ascii_whitespace().collect();
                    let wins: u32 = my
                        .split_whitespace()
                        .filter(|num| winning_nums.contains(num))
                        .count() as u32;

                    if wins == 0 {
                        return Some(0);
                    } else {
                        return Some(2u32.pow(wins - 1));
                    }
                }
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
        assert_eq!(solve(input), 13);
    }
}
