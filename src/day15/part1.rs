pub fn solve(input: &str) -> u32 {
    input
        .split(',')
        .map(|step| step.chars().fold(0, |acc, c| (acc + c as u32) * 17 % 256))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_riddle() {
        let input = include_str!("test_input1.txt");
        assert_eq!(solve(input), 1320);
    }
}
