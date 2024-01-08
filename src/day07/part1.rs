use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug)]
struct Hand {
    cards: String,
    bid: usize,
    hand_type: usize,
}

pub fn solve(input: &str) -> usize {
    let cards = "AKQJT98765432";

    let mut hands: Vec<Hand> = input
        .lines()
        .map(|line| {
            let elems: Vec<&str> = line.split_ascii_whitespace().collect();

            let bid = elems[1].parse().unwrap_or(0);
            let cards = elems[0].to_string();

            let mut map: HashMap<char, usize> = HashMap::new();

            for card in cards.chars() {
                *map.entry(card).or_insert(0) += 1;
            }

            let values: Vec<usize> = map.into_values().collect();

            let hand_type = if values.contains(&5) {
                7
            } else if values.contains(&4) {
                6
            } else if values.contains(&3) {
                if values.contains(&2) {
                    5
                } else {
                    4
                }
            } else if values.contains(&2) {
                if values.iter().filter(|&&x| x == 2).count() == 2 {
                    3
                } else {
                    2
                }
            } else {
                1
            };

            Hand {
                cards,
                bid,
                hand_type,
            }
        })
        .collect();

    hands.sort_by(|a, b| {
        a.hand_type.cmp(&b.hand_type).then_with(|| {
            for (a_card, b_card) in a.cards.chars().zip(b.cards.chars()) {
                let a_rank = cards
                    .chars()
                    .position(|ch| ch == a_card)
                    .unwrap_or(usize::MAX);
                let b_rank = cards
                    .chars()
                    .position(|ch| ch == b_card)
                    .unwrap_or(usize::MAX);
                if a_rank != b_rank {
                    return b_rank.cmp(&a_rank);
                }
            }
            Ordering::Equal
        })
    });

    hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| (idx + 1) * hand.bid)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_riddle() {
        let input = include_str!("test_input1.txt");
        assert_eq!(solve(input), 6440);
    }
}
