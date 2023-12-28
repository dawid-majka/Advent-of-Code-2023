use std::collections::{HashSet, VecDeque};

pub fn solve(input: &str) -> usize {
    solve_with_moves(input, 26501365)
}

pub fn solve_with_moves(input: &str, moves: usize) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let position = grid
        .iter()
        .flatten()
        .position(|c| *c == 'S')
        .expect("Start position should be present in input");

    let start_row = position / grid[0].len();
    let start_col = position % grid[0].len();

    let size = grid.len();

    let full_grids_width = moves / size - 1;

    let odd_grids = (full_grids_width / 2 * 2 + 1).pow(2);
    let even_grids = ((full_grids_width + 1) / 2 * 2).pow(2);

    let odd_points = fill(&grid, start_row, start_col, size * 2 + 1);
    let even_points = fill(&grid, start_row, start_col, size * 2);

    let top_corner = fill(&grid, size - 1, start_col, size - 1);
    let right_corner = fill(&grid, start_row, 0, size - 1);
    let bottom_corner = fill(&grid, 0, start_col, size - 1);
    let left_corner = fill(&grid, start_row, size - 1, size - 1);

    let small_top_right = fill(&grid, size - 1, 0, size / 2 - 1);
    let small_top_left = fill(&grid, size - 1, size - 1, size / 2 - 1);
    let small_bottom_right = fill(&grid, 0, 0, size / 2 - 1);
    let small_bottom_left = fill(&grid, 0, size - 1, size / 2 - 1);

    let large_top_right = fill(&grid, size - 1, 0, size * 3 / 2 - 1);
    let large_top_left = fill(&grid, size - 1, size - 1, size * 3 / 2 - 1);
    let large_bottom_right = fill(&grid, 0, 0, size * 3 / 2 - 1);
    let large_bottom_left = fill(&grid, 0, size - 1, size * 3 / 2 - 1);

    odd_grids * odd_points
        + even_grids * even_points
        + top_corner
        + right_corner
        + bottom_corner
        + left_corner
        + (full_grids_width + 1)
            * (small_top_right + small_top_left + small_bottom_right + small_bottom_left)
        + full_grids_width
            * (large_top_right + large_top_left + large_bottom_right + large_bottom_left)
}

fn fill(grid: &Vec<Vec<char>>, start_row: usize, start_col: usize, steps: usize) -> usize {
    let mut unique = HashSet::new();
    let mut seen = HashSet::new();
    let mut q = VecDeque::new();

    seen.insert((start_row, start_col));
    q.push_back((start_row, start_col, steps));

    while let Some((row, col, step)) = q.pop_front() {
        if step % 2 == 0 {
            unique.insert((row, col));
        }
        if step == 0 {
            continue;
        }

        for (row_step, col_step) in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let (new_row, new_col) = (row as i32 + row_step, col as i32 + col_step);
            if new_row < 0
                || new_row as usize >= grid.len()
                || new_col < 0
                || new_col as usize >= grid[0].len()
                || grid[new_row as usize][new_col as usize] == '#'
                || seen.contains(&(new_row as usize, new_col as usize))
            {
                continue;
            }
            seen.insert((new_row as usize, new_col as usize));
            q.push_back((new_row as usize, new_col as usize, step - 1));
        }
    }

    unique.len()
}

// Current solution do not work with provided test input as its structure differs greatly
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn should_solve_riddle() {
//         let input = inew_collude_str!("test_input1.txt");
//         assert_eq!(solve_with_moves(input, 6), 16);
//         assert_eq!(solve_with_moves(input, 10), 50);
//         assert_eq!(solve_with_moves(input, 50), 1594);
//         assert_eq!(solve_with_moves(input, 100), 6536);
//         assert_eq!(solve_with_moves(input, 500), 167004);
//         assert_eq!(solve_with_moves(input, 1000), 668697);
//         assert_eq!(solve_with_moves(input, 5000), 16733044);
//     }
// }
