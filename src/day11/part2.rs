#[derive(Debug, Clone)]
struct Galaxy {
    x: usize,
    y: usize,
}

pub fn solve(input: &str, expansion: isize) -> isize {
    let input: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();

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

    empty_rows.reverse();

    let mut empty_cols: Vec<usize> = Vec::new();

    for col_idx in 0..input.first().expect("Input should be valid.").len() {
        if input.iter().all(|row| row[col_idx] == '.') {
            empty_cols.push(col_idx);
        }
    }

    empty_cols.reverse();

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
            let mut distance: isize = (galaxy.x as isize - galaxy2.x as isize).abs()
                + (galaxy.y as isize - galaxy2.y as isize).abs();

            let x_range = galaxy.x.min(galaxy2.x)..=galaxy.x.max(galaxy2.x);
            let y_range = galaxy.y.min(galaxy2.y)..=galaxy.y.max(galaxy2.y);

            for col in empty_cols.iter() {
                if x_range.contains(col) {
                    distance += expansion - 1;
                }
            }

            for row in empty_rows.iter() {
                if y_range.contains(row) {
                    distance += expansion - 1;
                }
            }

            distance
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
        let expansion = 10;
        assert_eq!(solve(input, expansion), 1030);
    }

    #[test]
    fn should_solve_riddle2() {
        let input = include_str!("test_input1.txt");
        let expansion = 100;
        assert_eq!(solve(input, expansion), 8410);
    }
}
