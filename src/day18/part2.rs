pub fn solve(input: &str) -> usize {
    let instructions: Vec<(&str, usize)> = input
        .lines()
        .map(|line| {
            let mut hex = line
                .split_ascii_whitespace()
                .nth(2)
                .expect("Input to be valid.");

            hex = &hex[2..];

            let direction = &hex[5..6];
            let steps = &hex[..5];

            let steps = usize::from_str_radix(steps, 16).unwrap();

            (direction, steps)
        })
        .collect();

    let mut start = (0, 0);

    let mut vertices: Vec<(isize, isize)> = instructions
        .iter()
        .map(|(direction, steps)| {
            let mut m = match *direction {
                "0" => (0isize, 1),
                "2" => (0isize, -1),
                "3" => (-1, 0isize),
                "1" => (1, 0isize),
                _ => panic!("Invalid direction data"),
            };

            m.0 *= *steps as isize;
            m.1 *= *steps as isize;

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
        assert_eq!(solve(input), 952408144115);
    }
}
