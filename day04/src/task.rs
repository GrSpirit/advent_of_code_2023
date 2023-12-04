use std::str::FromStr;
use std::collections::HashSet;
use std::num::ParseIntError;

#[derive(thiserror::Error, Debug, Clone, Eq, PartialEq)]
pub enum Error {
    #[error("Parse int error")]
    ParseIntError(#[from] ParseIntError),
    #[error("Format error")]
    FormatError,
}

pub type Result<T> = std::result::Result<T, Error>;

struct Card {
    #[allow(dead_code)]
    id: u32,
    win: HashSet<u32>,
    got: Vec<u32>,
}

impl Card {
    fn score(&self) -> u32 {
        let count = self.got.iter().filter(|x| self.win.contains(x)).count() as u32;
        if count == 0 {
            return 0;
        }
        return 1 << (count - 1);
    }
    fn count(&self) -> usize {
        let count = self.got.iter().filter(|x| self.win.contains(x)).count();
        return count;
    }
}

impl FromStr for Card {
    type Err = Error;
    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let (card_str, nums) = s.split_once(':').ok_or(Error::FormatError)?;
        let id = card_str.split_ascii_whitespace().nth(1).ok_or(Error::FormatError)?.parse::<u32>()?;
        let (win_str, got_str) = nums.split_once('|').ok_or(Error::FormatError)?;
        let win = win_str.trim().split_ascii_whitespace().map(|n| n.parse::<u32>()).collect::<std::result::Result<HashSet<u32>, ParseIntError>>()?;
        let got = got_str.trim().split_ascii_whitespace().map(|n| n.parse::<u32>()).collect::<std::result::Result<Vec<u32>, ParseIntError>>()?;
        Ok(Card { id, win, got })
    }
}

pub fn task1<S: AsRef<str>>(lines: &[S]) -> Result<u32> {
    let cards = lines.iter().map(|s| s.as_ref().parse::<Card>()).collect::<Result<Vec<Card>>>()?;
    Ok(cards.into_iter().map(|c| c.score()).sum())
}

pub fn task2<S: AsRef<str>>(lines: &[S]) -> Result<u32> {
    let cards = lines.iter().map(|s| s.as_ref().parse::<Card>()).collect::<Result<Vec<Card>>>()?;
    let mut counts = vec![1; cards.len()];
    for i in 0..cards.len() {
        let win_count = cards[i].count();
        let n = counts[i];
        counts.iter_mut().skip(i + 1).take(win_count).for_each(|x| *x += n);
    }
    Ok(counts.into_iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str =
"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_task1() {
        let lines = DATA.lines().collect::<Vec<_>>();
        assert_eq!(Ok(13), task1(&lines));
    }

    #[test]
    fn test_task2() {
        let lines = DATA.lines().collect::<Vec<_>>();
        assert_eq!(Ok(30), task2(&lines));
    }

}
