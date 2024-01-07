use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
enum Spring {
    Operational,
    Demaged,
    Unknown,
}

pub fn solve(input: &str) -> usize {
    let springs: Vec<Vec<Spring>> = input
        .lines()
        .map(|line| {
            let line: Vec<&str> = line.split_whitespace().collect();

            let springs = line
                .first()
                .expect("Row should have broken groups data")
                .to_owned();
            let springs: Vec<&str> = std::iter::repeat(springs).take(5).collect();
            let springs = springs.join("?");

            springs
                .chars()
                .map(|c| match c {
                    '.' => Spring::Operational,
                    '#' => Spring::Demaged,
                    '?' => Spring::Unknown,
                    _ => panic!("Invalid element in springs data."),
                })
                .collect()
        })
        .collect();

    let broken_groups: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            let line: Vec<&str> = line.split_whitespace().collect();

            let groups = line
                .get(1)
                .expect("Row should have broken groups data")
                .to_owned();
            let groups: Vec<&str> = std::iter::repeat(groups).take(5).collect();
            let groups = groups.join(",");

            groups
                .split(',')
                .map(|c| {
                    c.parse::<usize>()
                        .expect("Broken springs data should be valid.")
                })
                .collect()
        })
        .collect();

    let mut cache: HashMap<(Vec<Spring>, Vec<usize>), usize> = HashMap::new();
    let mut count = 0;

    for i in 0..springs.len() {
        count += possible_solutions(&mut cache, &springs[i], &broken_groups[i]);
    }

    count
}

fn possible_solutions(
    cache: &mut HashMap<(Vec<Spring>, Vec<usize>), usize>,
    springs: &Vec<Spring>,
    broken_groups: &Vec<usize>,
) -> usize {
    if let Some(&result) = cache.get(&(springs.to_owned(), broken_groups.to_owned())) {
        return result;
    }

    if springs.is_empty() {
        if broken_groups.is_empty() {
            return 1;
        } else {
            return 0;
        }
    }

    if broken_groups.is_empty() {
        if springs.contains(&Spring::Demaged) {
            return 0;
        } else {
            return 1;
        }
    }

    let mut result = 0;

    if springs[0] == Spring::Operational || springs[0] == Spring::Unknown {
        result += possible_solutions(cache, &springs[1..].to_vec(), broken_groups);
    }

    if (springs[0] == Spring::Demaged || springs[0] == Spring::Unknown)
        && broken_groups[0] <= springs.len()
        && !springs[0..broken_groups[0]].contains(&Spring::Operational)
        && (springs.len() == broken_groups[0] || springs[broken_groups[0]] != Spring::Demaged)
    {
        result += possible_solutions(
            cache,
            &springs[(broken_groups[0] + 1).min(springs.len())..].to_vec(),
            &broken_groups[1..].to_vec(),
        );
    }

    cache.insert((springs.to_vec(), broken_groups.to_vec()), result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_riddle() {
        let input = include_str!("test_input1.txt");
        assert_eq!(solve(input), 525152);
    }
}
