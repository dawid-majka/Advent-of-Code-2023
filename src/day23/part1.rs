use std::collections::{HashMap, HashSet};

type Graph = HashMap<(usize, usize), HashMap<(usize, usize), usize>>;

pub fn solve(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let start = (
        0,
        grid[0]
            .iter()
            .position(|&c| c == '.')
            .expect("Start position should be valid."),
    );
    let end = (
        grid.len() - 1,
        grid[grid.len() - 1]
            .iter()
            .position(|&c| c == '.')
            .expect("End position should be valid."),
    );

    let mut points = vec![start, end];

    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, &col) in row.iter().enumerate() {
            if col == '#' {
                continue;
            }
            let mut neighbours = 0;
            for (r, c) in [
                (row_idx as isize - 1, col_idx as isize),
                (row_idx as isize + 1, col_idx as isize),
                (row_idx as isize, col_idx as isize - 1),
                (row_idx as isize, col_idx as isize + 1),
            ] {
                if r >= 0
                    && r < grid.len() as isize
                    && c >= 0
                    && c < grid[0].len() as isize
                    && grid[r as usize][c as usize] != '#'
                {
                    neighbours += 1;
                }
            }
            if neighbours >= 3 {
                points.push((row_idx, col_idx));
            }
        }
    }

    let mut graph: Graph = HashMap::new();

    let dirs: HashMap<char, Vec<(isize, isize)>> = [
        ('^', vec![(-1, 0)]),
        ('v', vec![(1, 0)]),
        ('<', vec![(0, -1)]),
        ('>', vec![(0, 1)]),
        ('.', vec![(-1, 0), (1, 0), (0, -1), (0, 1)]),
    ]
    .iter()
    .cloned()
    .collect();

    for &(sr, sc) in points.iter() {
        let mut stack = vec![(0, sr, sc)];
        let mut seen: HashSet<(usize, usize)> = HashSet::new();
        seen.insert((sr, sc));

        while let Some((n, r, c)) = stack.pop() {
            if n != 0 && points.contains(&(r, c)) {
                graph.entry((sr, sc)).or_default().insert((r, c), n);
                continue;
            }

            if let Some(directions) = dirs.get(&grid[r][c]) {
                for &(dr, dc) in directions {
                    let nr = r as isize + dr;
                    let nc = c as isize + dc;
                    if 0 <= nr
                        && nr < grid.len() as isize
                        && 0 <= nc
                        && nc < grid[0].len() as isize
                        && grid[nr as usize][nc as usize] != '#'
                        && !seen.contains(&(nr.try_into().unwrap(), nc.try_into().unwrap()))
                    {
                        stack.push((n + 1, nr.try_into().unwrap(), nc.try_into().unwrap()));
                        seen.insert((nr.try_into().unwrap(), nc.try_into().unwrap()));
                    }
                }
            }
        }
    }

    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    dfs(&start, &end, &graph, &mut seen).unwrap()
}

fn dfs(
    pt: &(usize, usize),
    end: &(usize, usize),
    graph: &Graph,
    seen: &mut HashSet<(usize, usize)>,
) -> Option<usize> {
    if pt == end {
        return Some(0);
    }

    seen.insert(*pt);
    let mut max_dist: Option<usize> = None;
    if let Some(neighbors) = graph.get(pt) {
        for (npt, &weight) in neighbors {
            if !seen.contains(npt) {
                if let Some(dist) = dfs(npt, end, graph, seen) {
                    let new_dist = dist + weight;
                    max_dist = match max_dist {
                        None => Some(new_dist),
                        Some(current_max) => Some(current_max.max(new_dist)),
                    };
                }
            }
        }
    }
    seen.remove(pt);

    max_dist
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_riddle() {
        let input = include_str!("test_input1.txt");
        assert_eq!(solve(input), 94);
    }
}
