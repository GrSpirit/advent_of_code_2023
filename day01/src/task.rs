#[derive(thiserror::Error, Debug, Clone, Copy, Eq, PartialEq)]
pub enum Error {
    #[error("Parse error")]
    ParseError
}

pub type Result<T> = std::result::Result<T, Error>;

fn extract_number(s: &str) -> Result<u32> {
    let first = s.bytes().find(|b| b.is_ascii_digit()).map(|b| (b - b'0') as u32).ok_or(Error::ParseError)?;
    let last = s.bytes().rev().find(|b| b.is_ascii_digit()).map(|b| (b - b'0') as u32).ok_or(Error::ParseError)?;
    Ok(first * 10 + last)
}

fn split_number(s: &str) -> Result<u32> {
    let literals = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let numbers = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let first = numbers.iter()
    .enumerate()
    .filter_map(|(x, n)| s.find(n).map(|i| (i, x)))
    .chain(
        literals.iter().enumerate().filter_map(|(x, n)| s.find(n).map(|i| (i, x)))
    ).min()
    .map(|(_, x)| x as u32).ok_or(Error::ParseError)?;

    let last = numbers.iter()
    .enumerate()
    .filter_map(|(x, n)| s.rfind(n).map(|i| (i, x)))
    .chain(
        literals.iter().enumerate().filter_map(|(x, n)| s.rfind(n).map(|i| (i, x)))
    ).max()
    .map(|(_, x)| x as u32).ok_or(Error::ParseError)?;
    return Ok(first * 10 + last);
}

pub fn task1<S: AsRef<str>>(lines: &[S]) -> Result<u32> {
    Ok(lines.into_iter().map(|s| extract_number(s.as_ref()).unwrap()).sum())
}

pub fn task2<S: AsRef<str>>(lines: &[S]) -> Result<u32> {
    Ok(lines.into_iter().map(|s| split_number(s.as_ref()).unwrap()).sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn extract_test() {
        assert_eq!(extract_number("12"), Ok(12));
        assert_eq!(extract_number("1a2"), Ok(12));
        assert_eq!(extract_number("a12b"), Ok(12));
        assert_eq!(extract_number("a1c2b"), Ok(12));
    }
    #[test]
    fn task1_test() {
        let data = "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet";
        assert_eq!(Ok(142), task1(&data.split('\n').collect::<Vec<_>>()));
    }
    #[test]
    fn split_test() {
        assert_eq!(split_number("9986fmfqhdmq8"), Ok(98));
        assert_eq!(split_number("31eight"), Ok(38));
        assert_eq!(split_number("vxnsvnine5seventhree2mksgsjbpkqdgbs"), Ok(92));
    }

    #[test]
    fn task2_test() {
        let data = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
        assert_eq!(task2(&data.split('\n').collect::<Vec<_>>()), Ok(281));
    }

    #[test]
    fn split2_test() {
        let data = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
        assert_eq!(data.split('\n').map(|s| split_number(s).unwrap()).collect::<Vec<_>>(), &[29, 83, 13, 24, 42, 14, 76]);
    }
}
