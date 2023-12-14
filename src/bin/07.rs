use core::panic;
use std::cmp::Ordering;
use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(7);

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
    combination: Combination,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
enum Card {
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

const NORMAL_ORDERING: [Card; 13] = [
    Card::Two,
    Card::Three,
    Card::Four,
    Card::Five,
    Card::Six,
    Card::Seven,
    Card::Eight,
    Card::Nine,
    Card::Ten,
    Card::Jack,
    Card::Queen,
    Card::King,
    Card::Ace,
];

const JOKER_ORDERING: [Card; 13] = [
    Card::Jack,
    Card::Two,
    Card::Three,
    Card::Four,
    Card::Five,
    Card::Six,
    Card::Seven,
    Card::Eight,
    Card::Nine,
    Card::Ten,
    Card::Queen,
    Card::King,
    Card::Ace,
];

impl Card {
    fn value(&self, ordering: [Card; 13]) -> u8 {
        return ordering.iter().position(|p| p.eq(self)).unwrap() as u8;
    }
}

impl From<char> for Card {
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

impl From<&[u8]> for Combination {
    // 5
    // 1 4
    // 2 3
    // 1 1 3
    // 1 2 2
    // 1 1 1 2
    // 1 1 1 1 1
    fn from(counts: &[u8]) -> Self {
        match counts {
            [5] => Self::FiveOfAKind,
            [2, 3] => Self::FullHouse,
            [.., 4] => Self::FourOfAKind,
            [.., 3] => Self::ThreeOfAKind,
            [.., 2, 2] => Self::TwoPair,
            [.., 2] => Self::OnePair,
            _ => Self::HighCard,
        }
    }
}

fn count_occurence(hand: &[Card]) -> Vec<(Card, u8)> {
    let mut cards: HashMap<Card, u8> = HashMap::new();

    for card in hand {
        cards
            .entry(*card)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    cards
        .iter()
        .sorted_by(|a, b| a.1.cmp(b.1))
        .map(|(card, count)| (card.to_owned(), count.to_owned()))
        .collect()
}

fn parse_hand(value: &str) -> Hand {
    let (cards, bid) = value.split_once(' ').expect("Should be valid hand");
    let bid = bid.parse().expect("should be a number");
    let cards: Vec<Card> = cards.chars().map(|ch| ch.into()).collect();
    let counts = count_occurence(cards.as_slice());
    let combination: Combination = counts
        .iter()
        .map(|(_, count)| count.to_owned())
        .collect::<Vec<u8>>()
        .as_slice()
        .into();

    Hand {
        cards,
        bid,
        combination,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands: Vec<Hand> = input.lines().map(parse_hand).collect();
    hands.sort_by(|a, b| match a.combination.cmp(&b.combination) {
        Ordering::Equal => {
            let tuples = a.cards.iter().interleave(b.cards.iter()).tuples();
            for (a, b) in tuples {
                match a.value(NORMAL_ORDERING).cmp(&b.value(NORMAL_ORDERING)) {
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

fn parse_joker_hand(value: &str) -> Hand {
    let (cards, bid) = value.split_once(' ').expect("Should be valid hand");
    let bid = bid.parse().expect("should be a number");
    let cards: Vec<Card> = cards.chars().map(|ch| ch.into()).collect();
    let mut counts = count_occurence(cards.as_slice());

    let unique = counts.len();

    if unique > 1 {
        if let Some(position) = counts.iter().position(|(card, _)| card == &Card::Jack) {
            let joker_count = counts.remove(position);
            counts[unique - 2].1 += joker_count.1;
        }
    }

    let combination: Combination = counts
        .iter()
        .map(|(_, count)| count.to_owned())
        .collect::<Vec<u8>>()
        .as_slice()
        .into();

    Hand {
        cards,
        bid,
        combination,
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hands: Vec<Hand> = input.lines().map(parse_joker_hand).collect();
    hands.sort_by(|a, b| match a.combination.cmp(&b.combination) {
        Ordering::Equal => {
            let tuples = a.cards.iter().interleave(b.cards.iter()).tuples();
            for (a, b) in tuples {
                match a.value(JOKER_ORDERING).cmp(&b.value(JOKER_ORDERING)) {
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
