use std::{num::ParseIntError, cmp::Ordering, collections::HashMap};
use itertools::Itertools;

#[derive(thiserror::Error, Debug, Clone, Eq, PartialEq)]
pub enum Error {
    #[error("Parse int error")]
    ParseIntError(#[from] ParseIntError),
    #[error("Format error")]
    FormatError,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Hand {
    HighCard,
    OnePair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}

fn hand(s: &str) -> Hand {
    let hm = s.bytes().fold(HashMap::new(), |mut acc, b| {
        acc.entry(b).and_modify(|count| *count += 1).or_insert(1);
        acc
    });
    let combinations = hm.into_values().sorted().rev().collect::<Vec<_>>();
    match (combinations[0], combinations.get(1)) {
        (5, _) => Hand::Five,
        (4, _) => Hand::Four,
        (3, Some(2)) => Hand::FullHouse,
        (3, _) => Hand::Three,
        (2, Some(2)) => Hand::TwoPair,
        (2, _) => Hand::OnePair,
        _ => Hand::HighCard
    }
}

fn hand_joker(s: &str) -> Hand {
    let jokers = s.bytes().filter(|b| *b == b'J').count();
    if jokers == 5 {
        return Hand::Five;
    }
    let h = hand(&String::from_utf8(s.bytes().filter(|b| *b != b'J').collect::<Vec<_>>()).unwrap());
    match (h, jokers) {
        (Hand::Five, _) => Hand::Five,
        (Hand::Four, 1) => Hand::Five,
        (Hand::Four, _) => Hand::Four,
        (Hand::FullHouse, _) => Hand::FullHouse,
        (Hand::Three, 2) => Hand::Five,
        (Hand::Three, 1) => Hand::Four,
        (Hand::Three, _) => Hand::Three,
        (Hand::TwoPair, 1) => Hand::FullHouse,
        (Hand::TwoPair, _) => Hand::TwoPair,
        (Hand::OnePair, 3) => Hand::Five,
        (Hand::OnePair, 2) => Hand::Four,
        (Hand::OnePair, 1) => Hand::Three,
        (Hand::OnePair, _) => Hand::OnePair,
        (Hand::HighCard, 4) => Hand::Five,
        (Hand::HighCard, 3) => Hand::Four,
        (Hand::HighCard, 2) => Hand::Three,
        (Hand::HighCard, 1) => Hand::OnePair,
        (Hand::HighCard, _) => Hand::HighCard,
    }
}

const CARD_ORDER: &[u8] = "23456789TJQKA".as_bytes();
const JOKER_ORDER: &[u8] = "J23456789TQKA".as_bytes();

fn cmp_card(lhs: u8, rhs: u8, order: &[u8]) -> Ordering {
    order.iter().find_position(|b| **b == lhs).unwrap().cmp(
        &order.iter().find_position(|b| **b == rhs).unwrap()
    )
}

fn compare_joker(lhs: &str, rhs: &str) -> Ordering {
    let hand_cmp = hand_joker(lhs).cmp(&hand_joker(rhs));
    hand_cmp.then_with(|| lhs.bytes().zip(rhs.bytes()).find_map(|(lhs, rhs)| {
        let card_cmp = cmp_card(lhs, rhs, JOKER_ORDER);
        if card_cmp.is_eq() {
            None
        } else {
            Some(card_cmp)
        }
    }).unwrap_or(Ordering::Equal))
}

fn compare(lhs: &str, rhs: &str) -> Ordering {
    let hand_cmp = hand(lhs).cmp(&hand(rhs));
    hand_cmp.then_with(|| lhs.bytes().zip(rhs.bytes()).find_map(|(lhs, rhs)| {
        let card_cmp = cmp_card(lhs, rhs, CARD_ORDER);
        if card_cmp.is_eq() {
            None
        } else {
            Some(card_cmp)
        }
    }).unwrap_or(Ordering::Equal))
}

fn common_task<S: AsRef<str>, F: Fn(&str, &str) -> Ordering>(lines: &[S], cmp: F) -> Result<u32> {
    let mut cards = Vec::with_capacity(lines.len());
    for line in lines {
        let (hand, bid) = line.as_ref().split_once(' ').ok_or(Error::FormatError)?;
        cards.push((hand.to_string(), bid.parse::<u32>()?));
    }
    cards.sort_by(|lhs, rhs| cmp(&lhs.0, &rhs.0));
    Ok(cards.into_iter().enumerate().fold(0, |score, (i, (_, bid))| score + (i as u32 + 1) * bid))

}

pub fn task1<S: AsRef<str>>(lines: &[S]) -> Result<u32> {
    common_task(lines, compare)
}

pub fn task2<S: AsRef<str>>(lines: &[S]) -> Result<u32> {
    common_task(lines, compare_joker)
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str =
"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_task1() {
        let lines = DATA.lines().collect::<Vec<_>>();
        assert_eq!(Ok(6440), task1(&lines));
    }

    #[test]
    fn test_task2() {
        let lines = DATA.lines().collect::<Vec<_>>();
        assert_eq!(Ok(5905), task2(&lines));
    }

    #[test]
    fn test_hand_joker() {
        assert_eq!(Hand::Five, hand_joker("JJJJ7"));
        assert_eq!(Hand::Five, hand_joker("JJJJJ"));
        assert_eq!(Hand::Five, hand_joker("JJJ88"));
        assert_eq!(Hand::Three, hand_joker("JJ867"));
    }
}
