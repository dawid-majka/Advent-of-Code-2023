use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
struct Brick {
    x1: u32,
    y1: u32,
    z1: u32,
    x2: u32,
    y2: u32,
    z2: u32,
}

impl Brick {
    fn overlaps(&self, other: &Brick) -> bool {
        self.x1.max(other.x1) <= self.x2.min(other.x2)
            && self.y1.max(other.y1) <= self.y2.min(other.y2)
    }
}

pub fn solve(input: &str) -> usize {
    let mut bricks: Vec<Brick> = input
        .lines()
        .map(|line| {
            let (one, two) = line
                .split_once('~')
                .expect("Input should have valid format.");

            let one: Vec<u32> = one
                .split(',')
                .map(|elem| {
                    elem.parse::<u32>()
                        .expect("Coordinates to be valid numbers")
                })
                .collect();
            let two: Vec<u32> = two
                .split(',')
                .map(|elem| {
                    elem.parse::<u32>()
                        .expect("Coordinates to be valid numbers")
                })
                .collect();

            Brick {
                x1: one[0],
                y1: one[1],
                z1: one[2],
                x2: two[0],
                y2: two[1],
                z2: two[2],
            }
        })
        .collect();

    bricks.sort_by_key(|brick| brick.z1);

    for index in 0..bricks.len() {
        let mut max_z = 1;

        let (left, right) = bricks.split_at_mut(index);
        let brick = &mut right[0];

        for base in left {
            if brick.overlaps(base) {
                max_z = max_z.max(base.z2 + 1);
            }
        }
        brick.z2 = brick.z2 + max_z - brick.z1;
        brick.z1 = max_z;
    }

    bricks.sort_by_key(|brick| brick.z1);

    let mut supports = vec![HashSet::new(); bricks.len()];
    let mut is_supported_by = vec![HashSet::new(); bricks.len()];

    for j in 0..bricks.len() {
        for i in 0..j {
            if bricks[i].overlaps(&bricks[j]) && bricks[j].z1 == bricks[i].z2 + 1 {
                supports[i].insert(j);
                is_supported_by[j].insert(i);
            }
        }
    }

    let mut total = 0;
    for i in 0..bricks.len() {
        let mut q: VecDeque<usize> = supports
            .get(i)
            .unwrap()
            .iter()
            .filter(|&&j| is_supported_by.get(j).map_or(false, |set| set.len() == 1))
            .cloned()
            .collect();

        let mut falling: HashSet<usize> = HashSet::from_iter(q.iter().cloned());
        falling.insert(i);

        while let Some(j) = q.pop_front() {
            for &k in supports.get(j).unwrap() {
                if !falling.contains(&k) {
                    let supporting_set = is_supported_by.get(k).unwrap();
                    if supporting_set.is_subset(&falling) {
                        q.push_back(k);
                        falling.insert(k);
                    }
                }
            }
        }

        total += falling.len() - 1;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_riddle() {
        let input = include_str!("test_input1.txt");
        assert_eq!(solve(input), 7);
    }
}
