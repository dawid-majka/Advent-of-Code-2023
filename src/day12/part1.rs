#[derive(Debug, PartialEq, Clone)]
enum Spring {
    Operational,
    Demaged,
    Unknown,
}

pub fn solve(input: &str) -> usize {
    let sum: usize = input
        .lines()
        .map(|line| {
            let line: Vec<&str> = line.split_whitespace().collect();

            let springs: Vec<Spring> = line
                .first()
                .expect("Row should have springs data")
                .chars()
                .map(|c| match c {
                    '.' => Spring::Operational,
                    '#' => Spring::Demaged,
                    '?' => Spring::Unknown,
                    _ => panic!("Invalid element in springs data."),
                })
                .collect();

            let broken_groups: Vec<usize> = line
                .get(1)
                .expect("Row should have broken groups data")
                .split(',')
                .map(|c| {
                    c.parse::<usize>()
                        .expect("Broken springs data should be valid.")
                })
                .collect();

            let mut possible_variants: Vec<Vec<Spring>> = Vec::new();

            let acc = Vec::new();

            generate_permutations(&springs, &acc, &mut possible_variants);

            let possible_variants: Vec<Vec<usize>> = possible_variants
                .iter()
                .map(|variant| {
                    group_springs(variant)
                        .iter()
                        .filter_map(|group| {
                            if group.1 == Spring::Demaged {
                                Some(group.0)
                            } else {
                                None
                            }
                        })
                        .collect()
                })
                .collect();

            let count = possible_variants
                .iter()
                .filter(|&variant| variant == &broken_groups)
                .count();

            count
        })
        .sum();

    sum
}

fn generate_permutations(current: &[Spring], acc: &[Spring], results: &mut Vec<Vec<Spring>>) {
    if current.is_empty() {
        results.push(acc.to_vec());
        return;
    }

    let (first, rest) = current.split_first().unwrap();

    match first {
        Spring::Unknown => {
            let mut acc_with_a = acc.to_vec();
            acc_with_a.push(Spring::Operational);
            generate_permutations(rest, &acc_with_a, results);

            let mut acc_with_b = acc.to_vec();
            acc_with_b.push(Spring::Demaged);
            generate_permutations(rest, &acc_with_b, results);
        }
        _ => {
            let mut acc_with_elem = acc.to_vec();
            acc_with_elem.push(first.clone());
            generate_permutations(rest, &acc_with_elem, results);
        }
    }
}

fn group_springs(variant: &[Spring]) -> Vec<(usize, Spring)> {
    let mut groups: Vec<(usize, Spring)> = Vec::new();

    let mut current = variant.first().expect("Springs to have valid data").clone();
    let mut count = 0;

    for elem in variant.iter() {
        if *elem == current {
            count += 1;
        } else {
            groups.push((count, current));
            count = 1;
            current = elem.clone();
        }
    }

    groups.push((count, current));
    groups
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_riddle() {
        let input = include_str!("test_input1.txt");
        assert_eq!(solve(input), 21);
    }
}
