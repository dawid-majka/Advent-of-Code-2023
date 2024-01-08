use core::panic;
use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug)]
struct Hand {
    cards: String,
    bid: usize,
    hand_type: usize,
}

enum HandType {
    Five = 7,
    Four = 6,
    Full = 5,
    Three = 4,
    TwoPairs = 3,
    Pair = 2,
    One = 1,
}

pub fn solve(input: &str) -> usize {
    let cards = "AKQT98765432J";

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

            let mut jokers = 0;

            if let Some(j) = map.get_mut(&'J') {
                jokers = *j;
                *j = 0
            }

            let mut values: Vec<usize> = map.into_values().collect();
            values.sort_by(|a, b| b.cmp(a));

            let first = values.first().unwrap_or(&0);
            let second = values.get(1).unwrap_or(&0);

            let hand_type = match (first, jokers) {
                (5, 0) => HandType::Five,
                (_, 5) => HandType::Five,

                (4, 1) => HandType::Five,
                (4, 0) => HandType::Four,

                (3, 2) => HandType::Five,
                (3, 1) => HandType::Four,
                (3, 0) => match second {
                    2 => HandType::Full,
                    _ => HandType::Three,
                },

                (2, 3) => HandType::Five,
                (2, 2) => HandType::Four,
                (2, 1) => match second {
                    2 => HandType::Full,
                    _ => HandType::Three,
                },
                (2, 0) => match second {
                    2 => HandType::TwoPairs,
                    _ => HandType::Pair,
                },

                (1, 4) => HandType::Five,
                (1, 3) => HandType::Four,
                (1, 2) => HandType::Three,
                (1, 1) => HandType::Pair,
                (1, 0) => HandType::One,
                (_, _) => panic!("Non existant hand type"),
            };

            Hand {
                cards,
                bid,
                hand_type: hand_type as usize,
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
        assert_eq!(solve(input), 5905);
    }
}
