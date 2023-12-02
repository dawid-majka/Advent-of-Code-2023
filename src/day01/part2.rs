pub fn solve(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut digits: Vec<u32> = vec![];

            for c in line.chars().enumerate() {
                let reminder = &line[c.0..];

                if reminder.starts_with("one") {
                    digits.push(1);
                } else if reminder.starts_with("two") {
                    digits.push(2);
                } else if reminder.starts_with("three") {
                    digits.push(3);
                } else if reminder.starts_with("four") {
                    digits.push(4);
                } else if reminder.starts_with("five") {
                    digits.push(5);
                } else if reminder.starts_with("six") {
                    digits.push(6);
                } else if reminder.starts_with("seven") {
                    digits.push(7);
                } else if reminder.starts_with("eight") {
                    digits.push(8);
                } else if reminder.starts_with("nine") {
                    digits.push(9);
                } else if let Some(digit) = c.1.to_digit(10) {
                    digits.push(digit);
                };
            }

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
        let input = include_str!("test_input2.txt");
        assert_eq!(solve(input), 281);
    }
}
