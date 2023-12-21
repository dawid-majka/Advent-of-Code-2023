#[derive(Debug, Clone, PartialEq)]
enum TileType {
    Vertical,
    Horizontal,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
    Empty,
    Start,
}

#[derive(Debug, Clone)]
struct Tile {
    x: usize,
    y: usize,
    tile_type: TileType,
}

#[derive(Debug, Clone)]
enum From {
    Left,
    Right,
    Up,
    Down,
}

pub fn solve(input: &str) -> usize {
    let grid: Vec<Vec<Tile>> = input
        .lines()
        .enumerate()
        .map(|(row_idx, row)| {
            row.chars()
                .enumerate()
                .map(|(col_idx, col)| match col {
                    '|' => Tile {
                        x: col_idx,
                        y: row_idx,
                        tile_type: TileType::Vertical,
                    },
                    '-' => Tile {
                        x: col_idx,
                        y: row_idx,
                        tile_type: TileType::Horizontal,
                    },
                    'L' => Tile {
                        x: col_idx,
                        y: row_idx,
                        tile_type: TileType::UpRight,
                    },
                    'J' => Tile {
                        x: col_idx,
                        y: row_idx,
                        tile_type: TileType::UpLeft,
                    },
                    '7' => Tile {
                        x: col_idx,
                        y: row_idx,
                        tile_type: TileType::DownLeft,
                    },
                    'F' => Tile {
                        x: col_idx,
                        y: row_idx,
                        tile_type: TileType::DownRight,
                    },
                    'S' => Tile {
                        x: col_idx,
                        y: row_idx,
                        tile_type: TileType::Start,
                    },
                    _ => Tile {
                        x: col_idx,
                        y: row_idx,
                        tile_type: TileType::Empty,
                    },
                })
                .collect()
        })
        .collect();

    let start_position = grid
        .iter()
        .enumerate()
        .find_map(|(_, row)| row.iter().find(|elem| elem.tile_type == TileType::Start))
        .expect("Start position should exist.");

    let top_max = 0;
    let bottom_max = grid.len() - 1;
    let left_max = 0;
    let right_max = grid.first().expect("Grid should have columns").len() - 1;

    let mut next_steps: Vec<(From, Tile)> = Vec::new();
    if start_position.y > top_max {
        next_steps.push((
            From::Down,
            grid[start_position.y - 1][start_position.x].clone(),
        ));
    }
    if start_position.y < bottom_max {
        next_steps.push((
            From::Up,
            grid[start_position.y + 1][start_position.x].clone(),
        ));
    }
    if start_position.x > left_max {
        next_steps.push((
            From::Right,
            grid[start_position.y][start_position.x - 1].clone(),
        ));
    }
    if start_position.y < right_max {
        next_steps.push((
            From::Left,
            grid[start_position.y][start_position.x + 1].clone(),
        ));
    }

    let mut steps_on_path: Vec<&Tile> = Vec::new();
    let mut next_step = start_position;
    let mut from = From::Left;

    for step in next_steps.iter() {
        if let Some((coming_from, (next_y, next_x))) = find_next_step(&step.0, &step.1) {
            steps_on_path.push(&step.1);
            from = coming_from;
            next_step = &grid[next_y][next_x];
            steps_on_path.push(next_step);
            break;
        }
    }

    loop {
        if let Some((coming_from, (next_y, next_x))) = find_next_step(&from, next_step) {
            if start_position.x == next_x && start_position.y == next_y {
                steps_on_path.push(start_position);
                break;
            }
            from = coming_from;
            next_step = &grid[next_y][next_x];
            steps_on_path.push(next_step);
        }
    }

    steps_on_path.len() / 2
}

fn find_next_step(from: &From, tile: &Tile) -> Option<(From, (usize, usize))> {
    match (from, &tile.tile_type) {
        (From::Left, TileType::Vertical) => None,
        (From::Left, TileType::Horizontal) => Some((From::Left, (tile.y, tile.x + 1))),
        (From::Left, TileType::UpRight) => None,
        (From::Left, TileType::UpLeft) => Some((From::Down, (tile.y - 1, tile.x))),
        (From::Left, TileType::DownRight) => None,
        (From::Left, TileType::DownLeft) => Some((From::Up, (tile.y + 1, tile.x))),
        (From::Right, TileType::Vertical) => None,
        (From::Right, TileType::Horizontal) => Some((From::Right, (tile.y, tile.x - 1))),
        (From::Right, TileType::UpRight) => Some((From::Down, (tile.y - 1, tile.x))),
        (From::Right, TileType::UpLeft) => None,
        (From::Right, TileType::DownRight) => Some((From::Up, (tile.y + 1, tile.x))),
        (From::Right, TileType::DownLeft) => None,
        (From::Up, TileType::Vertical) => Some((From::Up, (tile.y + 1, tile.x))),
        (From::Up, TileType::Horizontal) => None,
        (From::Up, TileType::UpRight) => Some((From::Left, (tile.y, tile.x + 1))),
        (From::Up, TileType::UpLeft) => Some((From::Right, (tile.y, tile.x - 1))),
        (From::Up, TileType::DownRight) => None,
        (From::Up, TileType::DownLeft) => None,
        (From::Down, TileType::Vertical) => Some((From::Down, (tile.y - 1, tile.x))),
        (From::Down, TileType::Horizontal) => None,
        (From::Down, TileType::UpRight) => None,
        (From::Down, TileType::UpLeft) => None,
        (From::Down, TileType::DownRight) => Some((From::Left, (tile.y, tile.x + 1))),
        (From::Down, TileType::DownLeft) => Some((From::Right, (tile.y, tile.x - 1))),

        (_, TileType::Empty) => None,
        (_, TileType::Start) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_riddle() {
        let input = include_str!("test_input1.txt");
        assert_eq!(solve(input), 4);
    }

    #[test]
    fn should_solve_riddle2() {
        let input = include_str!("test_input1a.txt");
        assert_eq!(solve(input), 8);
    }
}
