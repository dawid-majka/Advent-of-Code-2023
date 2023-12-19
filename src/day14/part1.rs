pub fn solve(input: &str) -> usize {
    let matrix: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();

    let row_len = matrix.len();
    let col_len = matrix[0].len();

    let mut transposed = vec![vec![' '; row_len]; col_len];

    for (i, row) in matrix.iter().enumerate() {
        for (j, &val) in row.iter().enumerate() {
            transposed[j][i] = val;
        }
    }

    let tilted: Vec<Vec<char>> = transposed
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
        .collect();

    let mut finished = vec![vec![' '; row_len]; col_len];

    for (i, row) in tilted.iter().enumerate() {
        for (j, &val) in row.iter().enumerate() {
            finished[j][i] = val;
        }
    }

    finished
        .iter()
        .enumerate()
        .map(|(row_idx, row)| {
            row.iter().filter(|c| c == &&'O').count() * (finished.len() - row_idx)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_riddle() {
        let input = include_str!("test_input1.txt");
        assert_eq!(solve(input), 136);
    }
}
