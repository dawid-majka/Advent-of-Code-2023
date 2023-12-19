#[derive(Debug, Clone)]
struct Galaxy {
    x: usize,
    y: usize,
}

pub fn solve(input: &str) -> isize {
    let mut input: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();

    let mut empty_rows: Vec<usize> = input
        .iter()
        .enumerate()
        .filter_map(|(row_idx, row)| {
            if row.iter().all(|&c| c == '.') {
                Some(row_idx)
            } else {
                None
            }
        })
        .collect();

    // Reverse indexes
    // If we start from lowest idx we would shift and following indexes would not match places where we need to duplicate.
    // If we start from the end order of indexes is kept
    empty_rows.reverse();

    let mut empty_cols: Vec<usize> = Vec::new();

    for col_idx in 0..input.first().expect("Input should be valid.").len() {
        if input.iter().all(|row| row[col_idx] == '.') {
            empty_cols.push(col_idx);
        }
    }

    empty_cols.reverse();

    for idx in empty_rows {
        input.insert(idx, input[idx].clone());
    }

    for idx in empty_cols {
        for row in input.iter_mut() {
            row.insert(idx, row[idx]);
        }
    }

    let galaxies: Vec<Galaxy> = input
        .iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(col_idx, col)| {
                    if *col == '#' {
                        Some(Galaxy {
                            y: row_idx,
                            x: col_idx,
                        })
                    } else {
                        None
                    }
                })
                .collect::<Vec<Galaxy>>()
        })
        .collect();

    let mut pairs: Vec<(Galaxy, Galaxy)> = Vec::new();

    for (idx, galaxy) in galaxies.iter().enumerate() {
        for galaxy2 in galaxies.iter().skip(idx + 1) {
            pairs.push((galaxy.clone(), galaxy2.clone()));
        }
    }

    let sum = pairs
        .iter()
        .map(|(galaxy, galaxy2)| {
            (galaxy.x as isize - galaxy2.x as isize).abs()
                + (galaxy.y as isize - galaxy2.y as isize).abs()
        })
        .sum();

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_riddle() {
        let input = include_str!("test_input1.txt");
        assert_eq!(solve(input), 374);
    }
}
