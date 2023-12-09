pub fn solve(input: &str) -> isize {
    let lines: Vec<Vec<isize>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|elem| elem.parse::<isize>().unwrap_or(0))
                .collect()
        })
        .collect();

    lines
        .iter()
        .map(|line| line.last().unwrap_or(&0) + next_value(line))
        .sum()
}

fn next_value(sequence: &[isize]) -> isize {
    if sequence.iter().all(|&x| x == 0) {
        return 0;
    }

    let mut new_sequence: Vec<isize> = Vec::new();

    for (idx, elem) in sequence.iter().enumerate() {
        if let Some(next) = sequence.get(idx + 1) {
            new_sequence.push(next - elem);
        }
    }

    new_sequence.last().unwrap_or(&0) + next_value(&new_sequence)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_riddle() {
        let input = include_str!("test_input1.txt");
        assert_eq!(solve(input), 114);
    }
}
