use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> u32 {
    let v: Vec<usize> = input
        .lines()
        .map(|line| {
            if let Some((card_id, numbers)) = line.split_once(": ") {
                if let Some((winning, my)) = numbers.split_once(" |") {
                    let winning_nums: HashSet<&str> = winning.split_ascii_whitespace().collect();
                    let wins: usize = my
                        .split_whitespace()
                        .filter(|num| winning_nums.contains(num))
                        .count();
                    return wins;
                }
            }
            0
        })
        .collect();

    let mut cards: Vec<usize> = vec![1; v.len()];

    v.iter().enumerate().for_each(|(idx, score)| {
        for i in idx + 1..=idx + score {
            let prev = *cards.get(idx).unwrap();

            if let Some(elem) = cards.get_mut(i) {
                *elem += prev;
            }
        }
    });

    cards.iter().sum::<usize>() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_riddle() {
        let input = include_str!("test_input2.txt");
        println!("{}", input);
        assert_eq!(solve(input), 30);
    }
}
