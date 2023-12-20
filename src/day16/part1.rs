use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Tile {
    x: isize,
    y: isize,
    tile_type: TileType,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum ComingFrom {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum TileType {
    Empty,
    VerticalSplit,
    HorizontalSplit,
    FrontTilt,
    BackTilt,
}

pub fn solve(input: &str) -> usize {
    let grid: Vec<Vec<Tile>> = input
        .lines()
        .enumerate()
        .map(|(row_idx, row)| {
            row.chars()
                .enumerate()
                .map(|(col_idx, col)| {
                    let tile_type = match col {
                        '.' => TileType::Empty,
                        '-' => TileType::HorizontalSplit,
                        '|' => TileType::VerticalSplit,
                        '/' => TileType::FrontTilt,
                        '\\' => TileType::BackTilt,
                        _ => panic!("Invalid data in input: r:{row_idx}, c:{col_idx}"),
                    };

                    Tile {
                        x: col_idx as isize,
                        y: row_idx as isize,
                        tile_type,
                    }
                })
                .collect()
        })
        .collect();

    let start_tile = &grid[0][0];
    let from = ComingFrom::Left;

    let mut energized: HashSet<Tile> = HashSet::new();
    let mut cache: HashMap<Tile, ComingFrom> = HashMap::new();

    next_step(&grid, start_tile, &from, &mut energized, &mut cache);

    energized.len()
}

fn next_step(
    grid: &Vec<Vec<Tile>>,
    tile: &Tile,
    from: &ComingFrom,
    energized: &mut HashSet<Tile>,
    cache: &mut HashMap<Tile, ComingFrom>,
) {
    energized.insert(tile.clone());
    cache.insert(tile.clone(), from.clone());

    let mut next: Vec<(ComingFrom, (isize, isize))> = Vec::new();

    match (from, &tile.tile_type) {
        (ComingFrom::Left, TileType::Empty) => {
            next.push((ComingFrom::Left, (tile.x + 1, tile.y)));
        }
        (ComingFrom::Left, TileType::VerticalSplit) => {
            next.push((ComingFrom::Down, (tile.x, tile.y - 1)));
            next.push((ComingFrom::Up, (tile.x, tile.y + 1)));
        }
        (ComingFrom::Left, TileType::HorizontalSplit) => {
            next.push((ComingFrom::Left, (tile.x + 1, tile.y)));
        }
        (ComingFrom::Left, TileType::FrontTilt) => {
            next.push((ComingFrom::Down, (tile.x, tile.y - 1)));
        }
        (ComingFrom::Left, TileType::BackTilt) => {
            next.push((ComingFrom::Up, (tile.x, tile.y + 1)));
        }

        (ComingFrom::Right, TileType::Empty) => {
            next.push((ComingFrom::Right, (tile.x - 1, tile.y)));
        }
        (ComingFrom::Right, TileType::VerticalSplit) => {
            next.push((ComingFrom::Down, (tile.x, tile.y - 1)));
            next.push((ComingFrom::Up, (tile.x, tile.y + 1)));
        }
        (ComingFrom::Right, TileType::HorizontalSplit) => {
            next.push((ComingFrom::Right, (tile.x - 1, tile.y)));
        }
        (ComingFrom::Right, TileType::FrontTilt) => {
            next.push((ComingFrom::Up, (tile.x, tile.y + 1)));
        }
        (ComingFrom::Right, TileType::BackTilt) => {
            next.push((ComingFrom::Down, (tile.x, tile.y - 1)));
        }

        (ComingFrom::Up, TileType::Empty) => {
            next.push((ComingFrom::Up, (tile.x, tile.y + 1)));
        }
        (ComingFrom::Up, TileType::VerticalSplit) => {
            next.push((ComingFrom::Up, (tile.x, tile.y + 1)));
        }
        (ComingFrom::Up, TileType::HorizontalSplit) => {
            next.push((ComingFrom::Left, (tile.x + 1, tile.y)));
            next.push((ComingFrom::Right, (tile.x - 1, tile.y)));
        }
        (ComingFrom::Up, TileType::FrontTilt) => {
            next.push((ComingFrom::Right, (tile.x - 1, tile.y)));
        }
        (ComingFrom::Up, TileType::BackTilt) => {
            next.push((ComingFrom::Left, (tile.x + 1, tile.y)));
        }

        (ComingFrom::Down, TileType::Empty) => {
            next.push((ComingFrom::Down, (tile.x, tile.y - 1)));
        }
        (ComingFrom::Down, TileType::VerticalSplit) => {
            next.push((ComingFrom::Down, (tile.x, tile.y - 1)));
        }
        (ComingFrom::Down, TileType::HorizontalSplit) => {
            next.push((ComingFrom::Left, (tile.x + 1, tile.y)));
            next.push((ComingFrom::Right, (tile.x - 1, tile.y)));
        }
        (ComingFrom::Down, TileType::FrontTilt) => {
            next.push((ComingFrom::Left, (tile.x + 1, tile.y)));
        }
        (ComingFrom::Down, TileType::BackTilt) => {
            next.push((ComingFrom::Right, (tile.x - 1, tile.y)));
        }
    }

    let max_x = grid.len() as isize;
    let max_y = grid[0].len() as isize;

    for (from, (x, y)) in next {
        if x < max_x && x >= 0 && y < max_y && y >= 0 {
            let next_tile = &grid[y as usize][x as usize];
            if let Some(c_from) = cache.get(next_tile) {
                if *c_from == from {
                    return;
                }
            }
            next_step(grid, next_tile, &from, energized, cache);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_riddle() {
        let input = include_str!("test_input1.txt");
        assert_eq!(solve(input), 46);
    }
}
