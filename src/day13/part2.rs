pub fn solve(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|pattern| {
            let pattern_rows: Vec<&str> = pattern.lines().collect();

            let r_middles: Vec<usize> = pattern_rows
                .iter()
                .enumerate()
                .filter_map(|(row_idx, row)| {
                    if row_idx < pattern_rows.len() - 1
                        && row
                            .chars()
                            .zip(pattern_rows[row_idx + 1].chars())
                            .filter(|(a, b)| a != b)
                            .count()
                            <= 1
                    {
                        return Some(row_idx);
                    }

                    None
                })
                .collect();

            for middle in r_middles.into_iter() {
                let length = if (0..middle + 1).len() <= ((middle + 1)..(pattern_rows.len())).len()
                {
                    middle + 1
                } else {
                    pattern_rows.len() - 1 - middle
                };

                let first_side: Vec<Vec<char>> = pattern_rows[middle + 1 - length..=middle]
                    .iter()
                    .map(|line| line.chars().collect::<Vec<char>>())
                    .collect();

                let mut second_side: Vec<Vec<char>> = pattern_rows[middle + 1..=middle + length]
                    .iter()
                    .map(|line| line.chars().collect::<Vec<char>>())
                    .collect();

                second_side.reverse();

                if first_side
                    .iter()
                    .flatten()
                    .zip(second_side.iter().flatten())
                    .filter(|(a, b)| a != b)
                    .count()
                    == 1
                {
                    return (middle + 1) * 100;
                }
            }

            let mut c_middles: Vec<usize> = Vec::new();

            let row_length = pattern_rows
                .first()
                .expect("Pattern should have rows")
                .len();

            for i in 0..row_length - 1 {
                if pattern_rows
                    .iter()
                    .filter(|row| row.as_bytes()[i] != row.as_bytes()[i + 1])
                    .count()
                    <= 1
                {
                    c_middles.push(i);
                }
            }

            for middle in c_middles.into_iter() {
                let length: usize = if (0..middle + 1).len() <= ((middle + 1)..(row_length)).len() {
                    middle + 1
                } else {
                    row_length - 1 - middle
                };

                if pattern_rows
                    .iter()
                    .filter(|row| {
                        let first_side: Vec<char> =
                            row[middle + 1 - length..=middle].chars().collect();

                        let mut second_side: Vec<char> =
                            row[middle + 1..=middle + length].chars().collect();
                        second_side.reverse();

                        first_side
                            .iter()
                            .zip(second_side.iter())
                            .filter(|(a, b)| a != b)
                            .count()
                            == 1
                    })
                    .count()
                    == 1
                {
                    return middle + 1;
                }
            }
            0
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_riddle() {
        let input = include_str!("test_input1.txt");
        assert_eq!(solve(input), 400);
    }

    #[test]
    fn should_solve_riddle2() {
        let input = include_str!("test_input2.txt");
        assert_eq!(solve(input), 407);
    }
}
