#[derive(Debug)]
struct Number {
    value: usize,
    start: usize,
    end: usize,
    row: usize,
}

pub fn solve(input: &str) -> usize {
    let mut stars: Vec<(usize, usize)> = Vec::new();

    let lines: Vec<&str> = input.lines().collect();

    let nums: Vec<Number> = lines
        .iter()
        .enumerate()
        .flat_map(|(row_idx, line)| {
            let mut start = 0;
            let mut digits: Vec<char> = Vec::new();

            let numbers: Vec<Number> = line
                .chars()
                .enumerate()
                .filter_map(|(idx, c)| {
                    if c.is_ascii_digit() {
                        if digits.is_empty() {
                            start = idx;
                        }
                        digits.push(c);
                    }

                    if c == '*' {
                        stars.push((row_idx, idx));
                    }

                    if c.is_ascii_digit() && idx == line.len() - 1
                        || !c.is_ascii_digit() && !digits.is_empty()
                    {
                        let value = digits
                            .iter()
                            .collect::<String>()
                            .parse::<usize>()
                            .unwrap_or(0);

                        let number = Number {
                            start,
                            value,
                            end: start + digits.len() - 1,
                            row: row_idx,
                        };

                        start = 0;
                        digits.clear();

                        return Some(number);
                    }

                    None
                })
                .collect();

            numbers
        })
        .collect();

    stars
        .iter()
        .filter_map(|star| {
            let row_range = star.0.saturating_sub(1)..=star.0 + 1;
            let col_range = star.1.saturating_sub(1)..=star.1 + 1;

            let neighbours: Vec<usize> = nums
                .iter()
                .filter_map(|number| {
                    if row_range.contains(&number.row)
                        && (number.start..=number.end).any(|i| col_range.contains(&i))
                    {
                        Some(number.value)
                    } else {
                        None
                    }
                })
                .collect();

            if neighbours.len() > 1 {
                Some(neighbours.iter().product::<usize>())
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_riddle() {
        let input = include_str!("test_input2.txt");
        assert_eq!(solve(input), 467835);
    }

    #[test]
    fn should_solve_riddle1a() {
        let input = include_str!("test_input2a.txt");
        assert_eq!(solve(input), 2558257);
    }
}

//84399773
