use std::collections::HashMap;

pub fn solve(input: &str) -> usize {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap_or("");

    lines.next();

    let map = lines.fold(HashMap::new(), |mut acc, line| {
        let (step, options) = line.split_once(" = ").unwrap();
        let options = options
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split_once(", ")
            .unwrap();

        acc.insert(step, options);
        acc
    });

    let mut steps = 0;
    let mut key = "AAA";
    let end = "ZZZ";

    for instruction in instructions.chars().cycle() {
        if let Some(options) = map.get(key) {
            key = match instruction {
                'L' => options.0,
                'R' => options.1,
                _ => panic!("Invalid instruction"),
            };
            steps += 1;
        }
        if key == end {
            break;
        }
    }

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_riddle() {
        let input = include_str!("test_input1.txt");
        assert_eq!(solve(input), 2);
    }

    #[test]
    fn should_solve_riddle2() {
        let input = include_str!("test_input1a.txt");
        assert_eq!(solve(input), 6);
    }
}
