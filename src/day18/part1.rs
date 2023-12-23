pub fn solve(input: &str) -> usize {
    let instructions: Vec<(char, isize)> = input
        .lines()
        .map(|line| {
            let mut elems = line.split_ascii_whitespace();

            let direction = elems
                .next()
                .expect("Input should be valid.")
                .chars()
                .next()
                .expect("Direction should be valid char");
            let steps = elems
                .next()
                .expect("Input should be valid.")
                .parse::<isize>()
                .expect("Steps count should be valid number");

            (direction, steps)
        })
        .collect();

    let mut start = (0, 0);

    let mut vertices: Vec<(isize, isize)> = instructions
        .iter()
        .map(|(direction, steps)| {
            let mut m = match direction {
                'R' => (0isize, 1),
                'L' => (0isize, -1),
                'U' => (-1, 0isize),
                'D' => (1, 0isize),
                _ => panic!("Invalid direction data"),
            };

            m.0 *= steps;
            m.1 *= steps;

            start.0 += m.0;
            start.1 += m.1;

            start
        })
        .collect();

    vertices.reverse();
    vertices.push((0, 0));

    let mut area = 0;
    let mut perimeter = 0;

    for i in 0..vertices.len() - 1 {
        let first = vertices[i];
        let second = vertices[i + 1];
        area += (first.0 * second.1) - (second.0 * first.1);
        perimeter += (second.0 - first.0).abs() + (second.1 - first.1).abs();
    }

    area = area.abs() / 2;

    let interior = area - (perimeter / 2) + 1;

    (interior + perimeter) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_riddle() {
        let input = include_str!("test_input1.txt");
        assert_eq!(solve(input), 62);
    }
}
