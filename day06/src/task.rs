use std::num::ParseIntError;
use itertools::Itertools;

#[derive(thiserror::Error, Debug, Clone, Eq, PartialEq)]
pub enum Error {
    #[error("Parse int error")]
    ParseIntError(#[from] ParseIntError),
    #[error("Format error")]
    FormatError,
}

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
fn winways_classic(time: u64, distance: u64) -> u64 {
    let mut way = 0;
    for i in 1..time {
        let d = (time - i) * i;
        if d > distance {
            way += 1;
        }
    }
    way
}

fn winways(time: u64, distance: u64) -> u64 {
    let mut l = 1;
    let mut r = time / 2;
    while l < r {
        let m = (l + r) / 2;
        let d = (time - m) * m;
        if d <= distance {
            l = m + 1;
        } else {
            r = m;
        }
    }
    return time + 1 - l * 2;
}

pub fn task1<S: AsRef<str>>(lines: &[S]) -> Result<u64> {
    let time = lines.get(0)
        .ok_or(Error::FormatError)?
        .as_ref()
        .split_whitespace()
        .skip(1)
        .map(|n| n.parse::<u64>())
        .collect::<std::result::Result<Vec<_>, ParseIntError>>()?;
    let distance = lines.get(1)
        .ok_or(Error::FormatError)?
        .as_ref()
        .split_whitespace()
        .skip(1)
        .map(|n| n.parse::<u64>())
        .collect::<std::result::Result<Vec<_>, ParseIntError>>()?;
    if time.len() != distance.len() {
        return Err(Error::FormatError);
    }
    let n = time.len();
    let mut ans = 1;
    for attempt in 0..n {
        ans *= winways(time[attempt], distance[attempt]);
    }
    Ok(ans)
}

pub fn task2<S: AsRef<str>>(lines: &[S]) -> Result<u64> {
    let time: u64 = lines
        .get(0)
        .ok_or(Error::FormatError)?
        .as_ref()
        .split_whitespace()
        .skip(1)
        .join("")
        .parse()?;
    let distance: u64 = lines
        .get(1)
        .ok_or(Error::FormatError)?
        .as_ref()
        .split_whitespace()
        .skip(1)
        .join("")
        .parse()?;
    Ok(winways(time, distance))
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str =
"Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_task1() {
        let lines = DATA.lines().collect::<Vec<_>>();
        assert_eq!(Ok(288), task1(&lines));
    }

    #[test]
    fn test_task2() {
        let lines = DATA.lines().collect::<Vec<_>>();
        assert_eq!(Ok(71503), task2(&lines));
    }

    #[test]
    fn test_winways() {
        assert_eq!(winways_classic(7, 9), winways(7, 9));
        assert_eq!(winways_classic(15, 40), winways(15, 40));
        assert_eq!(winways_classic(30, 200), winways(30, 200));
    }
}
