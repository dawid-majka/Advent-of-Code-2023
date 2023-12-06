#[derive(Debug)]
struct Race {
    time: u32,
    distance: u32,
}

impl Race {
    fn find_range_count(&self) -> usize {
        (0..self.time)
            .filter_map(|speed| {
                let distance = (self.time - speed) * speed;
                if distance > self.distance {
                    Some(distance)
                } else {
                    None
                }
            })
            .count()
    }
}

pub fn solve(input: &str) -> usize {
    let lines: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.split_once(':')
                .unwrap()
                .1
                .split_ascii_whitespace()
                .map(|digit| digit.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    let times = lines.get(0).unwrap();
    let distances = lines.get(1).unwrap();

    let races: Vec<Race> = times
        .iter()
        .zip(distances.iter())
        .map(|(&time, &distance)| Race { time, distance })
        .collect();

    races.iter().map(|race| race.find_range_count()).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_riddle() {
        let input = include_str!("test_input1.txt");
        assert_eq!(solve(input), 288);
    }
}
