use std::collections::HashMap;

pub fn solve(input: &str) -> usize {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap_or("");

    lines.next();

    let mut starting_points = Vec::new();

    let map = lines.fold(HashMap::new(), |mut acc, line| {
        let (step, options) = line.split_once(" = ").unwrap();
        let options = options
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split_once(", ")
            .unwrap();

        if step.ends_with('A') {
            starting_points.push(step);
        }

        acc.insert(step, options);
        acc
    });

    let mut steps = Vec::new();

    for key in starting_points.iter() {
        let mut key = *key;

        for (step, instruction) in instructions.chars().cycle().enumerate() {
            if key.ends_with('Z') {
                steps.push(step);
                break;
            }

            if let Some(options) = map.get(key) {
                key = match instruction {
                    'L' => options.0,
                    'R' => options.1,
                    _ => panic!("Invalid instruction"),
                };
            }
        }
    }

    lcm(&steps)
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_riddle() {
        let input = include_str!("test_input2.txt");
        assert_eq!(solve(input), 6);
    }
}
