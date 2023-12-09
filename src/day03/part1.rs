#[derive(Debug)]
struct Number {
    value: usize,
    start: usize,
    end: usize,
}

pub fn solve(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();

    lines
        .iter()
        .enumerate()
        .map(|(idx, line)| {
            let mut start = 0;
            let mut digits: Vec<char> = Vec::new();

            let numbers: Vec<Number> = line
                .chars()
                .enumerate()
                .filter_map(|(col_idx, c)| {
                    if c.is_ascii_digit() {
                        if digits.is_empty() {
                            start = col_idx;
                        }
                        digits.push(c);
                    }

                    if c.is_ascii_digit() && col_idx == line.len() - 1
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
                        };

                        start = 0;
                        digits.clear();

                        return Some(number);
                    }

                    None
                })
                .collect();

            let parts_sum: usize = numbers
                .iter()
                .filter_map(|number| {
                    let mut value = 0;

                    for row_idx in idx.saturating_sub(1)..=idx + 1 {
                        for col_idx in number.start.saturating_sub(1)..=(number.end + 1) {
                            if let Some(line) = lines.get(row_idx) {
                                if let Some(elem) = line.chars().nth(col_idx) {
                                    if !elem.is_ascii_digit() && elem != '.' {
                                        value = number.value;
                                        break;
                                    }
                                }
                            };
                        }
                        if value != 0 {
                            break;
                        }
                    }
                    if value != 0 {
                        Some(value)
                    } else {
                        None
                    }
                })
                .sum();

            parts_sum
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_riddle() {
        let input = include_str!("test_input1.txt");
        assert_eq!(solve(input), 4361);
    }

    #[test]
    fn should_solve_riddle1a() {
        let input = include_str!("test_input1a.txt");
        assert_eq!(solve(input), 26302);
    }
}
