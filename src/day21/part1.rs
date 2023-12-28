use std::collections::HashSet;

pub fn solve(input: &str) -> usize {
    solve_with_moves(input, 64)
}

pub fn solve_with_moves(input: &str, moves: usize) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let valid_steps: Vec<Vec<Vec<(usize, usize)>>> = grid
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, _)| get_valid_steps(&y, &x, &grid))
                .collect()
        })
        .collect();

    let position = grid
        .iter()
        .flatten()
        .position(|c| *c == 'S')
        .expect("Start position should be present in input");
    let position = (position / grid[0].len(), position % grid[0].len());

    let mut steps = vec![position];

    for _ in 0..moves {
        let mut steps_set = HashSet::new();

        for step in steps {
            steps_set.extend(valid_steps[step.0][step.1].iter());
        }

        steps = steps_set.into_iter().collect();
    }

    steps.len()
}

fn get_valid_steps(y: &usize, x: &usize, grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut valid_steps = Vec::new();

    let moves = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    for step in moves.iter() {
        let y: isize = *y as isize + step.0;
        let x: isize = *x as isize + step.1;

        if y >= 0 && y < grid.len() as isize && x >= 0 && x < grid[0].len() as isize {
            let x = x as usize;
            let y = y as usize;
            if grid[y][x] != '#' {
                valid_steps.push((y, x));
            }
        }
    }
    valid_steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_riddle() {
        let input = include_str!("test_input1.txt");
        assert_eq!(solve_with_moves(input, 6), 16);
    }
}
