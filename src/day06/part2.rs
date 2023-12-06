#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
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
    let lines: Vec<usize> = input
        .lines()
        .map(|line| {
            let line: String = line
                .split_once(':')
                .unwrap()
                .1
                .split_ascii_whitespace()
                .inspect(|elem| println!("{}", elem))
                .collect();

            line.parse::<usize>().unwrap_or(0)
        })
        .collect();

    let time = lines[0];
    let distance = lines[1];

    let race = Race { time, distance };

    race.find_range_count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_riddle() {
        let input = include_str!("test_input1.txt");
        assert_eq!(solve(input), 71503);
    }
}
