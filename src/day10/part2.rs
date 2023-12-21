#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, Eq, PartialEq)]
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

#[derive(Debug, PartialEq)]
enum Open {
    Up,
    Down,
    None,
}

pub fn solve(input: &str) -> usize {
    let mut grid: Vec<Vec<Tile>> = input
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
        .expect("Start position should exist.")
        .clone();

    let path = find_path_elements_and_replace_start(&start_position, &mut grid);

    grid.iter()
        .map(|row| {
            let mut intersections = 0;
            let mut inside_count = 0;

            let mut open = Open::None;

            row.iter().for_each(|col| {
                if path.contains(col) {
                    if col.tile_type == TileType::Vertical {
                        intersections += 1;
                    }

                    if open == Open::None {
                        if col.tile_type == TileType::DownRight {
                            open = Open::Down
                        }

                        if col.tile_type == TileType::UpRight {
                            open = Open::Up
                        }
                    }

                    if open == Open::Down {
                        if col.tile_type == TileType::UpLeft {
                            intersections += 1;
                            open = Open::None;
                        }

                        if col.tile_type == TileType::DownLeft {
                            open = Open::None;
                        }
                    }

                    if open == Open::Up {
                        if col.tile_type == TileType::UpLeft {
                            open = Open::None;
                        }

                        if col.tile_type == TileType::DownLeft {
                            intersections += 1;
                            open = Open::None;
                        }
                    }
                } else if intersections % 2 != 0 {
                    inside_count += 1;
                }
            });
            inside_count
        })
        .sum()
}

fn find_path_elements_and_replace_start(
    start_position: &Tile,
    grid: &mut Vec<Vec<Tile>>,
) -> Vec<Tile> {
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

    let mut path: Vec<Tile> = Vec::new();
    let mut next_step = start_position;
    let mut from = From::Left;

    let mut valid_steps_froms: Vec<From> = Vec::new();

    for step in next_steps.iter() {
        if let Some((_, _)) = find_next_step(&step.0, &step.1) {
            valid_steps_froms.push(step.0.clone());
        }
    }

    for step in next_steps.iter() {
        if let Some((coming_from, (next_y, next_x))) = find_next_step(&step.0, &step.1) {
            path.push(step.1.clone());
            from = coming_from;
            next_step = &grid[next_y][next_x];
            path.push(next_step.clone());
            break;
        }
    }

    loop {
        if let Some((coming_from, (next_y, next_x))) = find_next_step(&from, next_step) {
            if start_position.x == next_x && start_position.y == next_y {
                break;
            }
            from = coming_from;
            next_step = &grid[next_y][next_x];
            path.push(next_step.clone());
        }
    }

    replace_s(grid, start_position, &valid_steps_froms);

    let new_start = grid[start_position.y][start_position.x].clone();
    path.push(new_start);

    path
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

fn replace_s(grid: &mut [Vec<Tile>], start_position: &Tile, s_froms: &[From]) {
    let tile_type = match (&s_froms[0], &s_froms[1]) {
        (From::Left, From::Right) => TileType::Vertical,
        (From::Right, From::Left) => TileType::Vertical,
        (From::Up, From::Down) => TileType::Horizontal,
        (From::Down, From::Up) => TileType::Horizontal,
        (From::Left, From::Up) => TileType::DownRight,
        (From::Up, From::Left) => TileType::DownRight,
        (From::Right, From::Up) => TileType::DownLeft,
        (From::Up, From::Right) => TileType::DownLeft,
        (From::Down, From::Left) => TileType::UpRight,
        (From::Left, From::Down) => TileType::UpRight,
        (From::Down, From::Right) => TileType::UpLeft,
        (From::Right, From::Down) => TileType::UpLeft,
        (_, _) => panic!("Unreachable."),
    };

    let tile = Tile {
        x: start_position.x,
        y: start_position.y,
        tile_type,
    };

    grid[start_position.y][start_position.x] = tile;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_riddle() {
        let input = include_str!("test_input2.txt");
        assert_eq!(solve(input), 4);
    }

    #[test]
    fn should_solve_riddle2() {
        let input = include_str!("test_input2a.txt");
        assert_eq!(solve(input), 8);
    }

    #[test]
    fn should_solve_riddle3() {
        let input = include_str!("test_input2b.txt");
        assert_eq!(solve(input), 10);
    }
}
