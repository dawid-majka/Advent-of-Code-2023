use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    cost: usize,
    x: isize,
    y: isize,
    dx: isize,
    dy: isize,
    steps: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl State {
    fn new(cost: usize, x: isize, y: isize, dx: isize, dy: isize, steps: usize) -> Self {
        State {
            cost,
            x,
            y,
            dx,
            dy,
            steps,
        }
    }
}

pub fn solve(input: &str) -> usize {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("Input should have valid structure."))
                .collect()
        })
        .collect();

    let mut distances = HashMap::new();

    let mut heap = BinaryHeap::new();
    heap.push(State::new(0, 0, 0, 0, 0, 0));

    let max_x = (grid[0].len() - 1) as isize;
    let max_y = (grid.len() - 1) as isize;

    while let Some(state) = heap.pop() {
        if state.y == max_y && state.x == max_x && state.steps >= 4 {
            return state.cost;
        }

        if let Some(cost) = distances.get(&(state.x, state.y, state.dx, state.dy, state.steps)) {
            if state.cost >= *cost {
                continue;
            }
        }

        distances.insert(
            (state.x, state.y, state.dx, state.dy, state.steps),
            state.cost,
        );

        if state.steps < 10 && (state.dy, state.dx) != (0, 0) {
            let new_y = state.y + state.dy;
            let new_x = state.x + state.dx;

            if new_x >= 0 && new_x <= max_x && new_y >= 0 && new_y <= max_y {
                heap.push(State::new(
                    state.cost + grid[new_y as usize][new_x as usize] as usize,
                    new_x,
                    new_y,
                    state.dx,
                    state.dy,
                    state.steps + 1,
                ));
            }
        }

        if state.steps >= 4 || (state.dy, state.dx) == (0, 0) {
            for (new_dx, new_dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                if (new_dy, new_dx) != (state.dy, state.dx)
                    && (new_dy, new_dx) != (-state.dy, -state.dx)
                {
                    let new_y = state.y + new_dy;
                    let new_x = state.x + new_dx;

                    if new_y >= 0 && new_y <= max_y && new_x >= 0 && new_x <= max_x {
                        heap.push(State::new(
                            state.cost + grid[new_y as usize][new_x as usize] as usize,
                            new_x,
                            new_y,
                            new_dx,
                            new_dy,
                            1,
                        ));
                    }
                }
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_riddle() {
        let input = include_str!("test_input1.txt");
        assert_eq!(solve(input), 94);
    }

    #[test]
    fn should_solve_riddle2() {
        let input = include_str!("test_input2.txt");
        assert_eq!(solve(input), 71);
    }
}
