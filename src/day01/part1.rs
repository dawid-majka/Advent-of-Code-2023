pub fn solve(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let digits = line
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>();

            let first = digits.first().expect("should be a digit");
            let last = digits.last().expect("should be a digit");
            format!("{}{}", first, last)
                .parse::<u32>()
                .expect("should be a number")
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_riddle() {
        let input = include_str!("test_input1.txt");
        assert_eq!(solve(input), 142);
    }
}
