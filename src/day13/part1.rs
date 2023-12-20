pub fn solve(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|pattern| {
            let pattern_rows: Vec<&str> = pattern.lines().collect();

            let r_middles: Vec<usize> = pattern_rows
                .iter()
                .enumerate()
                .filter_map(|(row_idx, _)| {
                    if row_idx < pattern_rows.len() - 1
                        && pattern_rows[row_idx] == pattern_rows[row_idx + 1]
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

                let first_side = pattern_rows[middle + 1 - length..=middle].to_owned();

                let mut second_side = pattern_rows[middle + 1..=middle + length].to_owned();
                second_side.reverse();

                if first_side == second_side {
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
                    .all(|row| row.as_bytes()[i] == row.as_bytes()[i + 1])
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

                if pattern_rows.iter().all(|row| {
                    let first_side: Vec<char> = row[middle + 1 - length..=middle].chars().collect();

                    let mut second_side: Vec<char> =
                        row[middle + 1..=middle + length].chars().collect();
                    second_side.reverse();

                    first_side == second_side
                }) {
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
        assert_eq!(solve(input), 405);
    }
}
