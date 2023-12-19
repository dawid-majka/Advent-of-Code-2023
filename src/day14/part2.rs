use std::collections::HashMap;

pub fn solve(input: &str) -> usize {
    let mut matrix: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();

    let mut cache = HashMap::new();

    for i in 0..1000000000 {
        if let Some(appeared) = cache.insert(matrix.clone(), i) {
            if (1000000000 - i) % (i - appeared) == 0 {
                break;
            }
        }
        cycle(&mut matrix);
    }

    matrix
        .iter()
        .enumerate()
        .map(|(row_idx, row)| row.iter().filter(|c| c == &&'O').count() * (matrix.len() - row_idx))
        .sum()
}

fn cycle(matrix: &mut Vec<Vec<char>>) {
    // North
    transpose(matrix);
    tilt_left(matrix);
    transpose(matrix);

    // West
    tilt_left(matrix);

    // South
    transpose(matrix);
    tilt_right(matrix);
    transpose(matrix);

    // East
    tilt_right(matrix);
}

fn transpose(matrix: &mut Vec<Vec<char>>) {
    let row_len = matrix.len();
    let col_len = matrix.first().map_or(0, Vec::len);

    for i in 0..row_len {
        for j in i + 1..col_len {
            let temp = matrix[i][j];
            matrix[i][j] = matrix[j][i];
            matrix[j][i] = temp;
        }
    }
}

fn tilt_left(matrix: &mut Vec<Vec<char>>) {
    *matrix = matrix
        .iter()
        .map(|row| {
            row.split(|c| c == &'#')
                .flat_map(|chunk| {
                    let o_count = chunk.iter().filter(|c: &&char| c == &&'O').count();
                    std::iter::repeat('O')
                        .take(o_count)
                        .chain(std::iter::repeat('.').take(chunk.len() - o_count))
                        .chain(std::iter::once('#'))
                })
                .take(row.len())
                .collect()
        })
        .collect()
}

fn tilt_right(matrix: &mut Vec<Vec<char>>) {
    *matrix = matrix
        .iter()
        .map(|row| {
            row.split(|c| c == &'#')
                .flat_map(|chunk| {
                    let o_count = chunk.iter().filter(|c: &&char| c == &&'O').count();
                    std::iter::repeat('.')
                        .take(chunk.len() - o_count)
                        .chain(std::iter::repeat('O').take(o_count))
                        .chain(std::iter::once('#'))
                })
                .take(row.len())
                .collect()
        })
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn should_solve_riddle_one_cycle() {
    //     let input = include_str!("test_input1.txt");
    //     assert_eq!(solve(input), 87);
    // }

    // #[test]
    // fn should_solve_riddle_three_cycles() {
    //     let input = include_str!("test_input1.txt");
    //     assert_eq!(solve(input), 69);
    // }

    #[test]
    fn should_solve_riddle_full() {
        let input = include_str!("test_input1.txt");
        assert_eq!(solve(input), 64);
    }
}
