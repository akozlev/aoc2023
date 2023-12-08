use core::panic;
use std::cmp::Ordering;
use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(7);

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
enum JokerCard {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl From<char> for JokerCard {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Joker,
            'T' => Self::Ten,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            _ => panic!("Unexpected charecter for card"),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
enum NormalCard {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl From<char> for NormalCard {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Jack,
            'T' => Self::Ten,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            _ => panic!("Unexpected charecter for card"),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Combination {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl From<&[JokerCard]> for Combination {
    fn from(hand: &[JokerCard]) -> Self {
        let mut cards: HashMap<JokerCard, u8> = HashMap::new();

        for card in hand {
            cards
                .entry(*card)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        let jokers = cards.remove(&JokerCard::Joker);

        let mut card_counts: Vec<u8> = cards
            .values()
            .sorted()
            .map(|count| count.to_owned())
            .collect();

        if let Some(count) = jokers {
            if let Some(last) = card_counts.last_mut() {
                *last += count;
            } else {
                card_counts.push(count);
            }
        }

        // 5
        // 1 4
        // 2 3
        // 1 1 3
        // 1 2 2
        // 1 1 1 2
        // 1 1 1 1 1
        match card_counts.len() {
            1 => Self::FiveOfAKind,
            2 => {
                if card_counts[0] == 1 {
                    Self::FourOfAKind
                } else {
                    Self::FullHouse
                }
            }
            3 => {
                if let Some(3) = card_counts.last() {
                    Self::ThreeOfAKind
                } else {
                    Self::TwoPair
                }
            }
            4 => Self::OnePair,
            5 => Self::HighCard,
            _ => panic!("Hand doesn't have 5 cards"),
        }
    }
}
impl From<&[NormalCard]> for Combination {
    fn from(hand: &[NormalCard]) -> Self {
        let mut cards: HashMap<NormalCard, u8> = HashMap::new();

        for &card in hand {
            cards
                .entry(card)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        let card_counts: Vec<u8> = cards
            .values()
            .sorted()
            .map(|count| count.to_owned())
            .collect();

        // 5
        // 1 4
        // 2 3
        // 1 1 3
        // 1 2 2
        // 1 1 1 2
        // 1 1 1 1 1
        match card_counts.len() {
            1 => Self::FiveOfAKind,
            2 => {
                if card_counts[0] == 1 {
                    Self::FourOfAKind
                } else {
                    Self::FullHouse
                }
            }
            3 => {
                if let Some(3) = card_counts.last() {
                    Self::ThreeOfAKind
                } else {
                    Self::TwoPair
                }
            }
            4 => Self::OnePair,
            5 => Self::HighCard,
            _ => panic!("Hand doesn't have 5 cards"),
        }
    }
}

#[derive(Debug)]
struct Hand<T> {
    cards: Vec<T>,
    bid: u32,
    combination: Combination,
}

impl From<&str> for Hand<JokerCard> {
    fn from(value: &str) -> Self {
        let (cards, bid) = value.split_once(' ').expect("Should be valid hand");
        let bid = bid.parse().expect("should be a number");
        let cards: Vec<JokerCard> = cards.chars().map(|ch| ch.into()).collect();
        let combination = cards.as_slice().into();

        Hand {
            cards,
            bid,
            combination,
        }
    }
}

impl From<&str> for Hand<NormalCard> {
    fn from(value: &str) -> Self {
        let (cards, bid) = value.split_once(' ').expect("Should be valid hand");
        let bid = bid.parse().expect("should be a number");
        let cards: Vec<NormalCard> = cards.chars().map(|ch| ch.into()).collect();
        let combination = cards.as_slice().into();

        Hand {
            cards,
            bid,
            combination,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands: Vec<Hand<NormalCard>> = input.lines().map(|line| line.into()).collect();
    hands.sort_by(|a, b| match a.combination.cmp(&b.combination) {
        Ordering::Equal => {
            let tuples = a.cards.iter().interleave(b.cards.iter()).tuples();
            for (a, b) in tuples {
                match a.cmp(b) {
                    Ordering::Equal => continue,
                    other => return other,
                }
            }

            Ordering::Equal
        }
        other => other,
    });

    Some(
        hands
            .iter()
            .enumerate()
            .map(|(index, hand)| hand.bid * (index as u32 + 1))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hands: Vec<Hand<JokerCard>> = input.lines().map(|line| line.into()).collect();
    hands.sort_by(|a, b| match a.combination.cmp(&b.combination) {
        Ordering::Equal => {
            let tuples = a.cards.iter().interleave(b.cards.iter()).tuples();
            for (a, b) in tuples {
                match a.cmp(b) {
                    Ordering::Equal => continue,
                    other => return other,
                }
            }

            Ordering::Equal
        }
        other => other,
    });

    Some(
        hands
            .iter()
            .enumerate()
            .map(|(index, hand)| hand.bid * (index as u32 + 1))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
