use std::str::FromStr;

#[derive(thiserror::Error, Debug, Clone, Eq, PartialEq)]
pub enum Error {
    #[error("no data")]
    NoData,
    #[error("parse error")]
    ParseError,
}

pub type Result<T> = std::result::Result<T, Error>;

enum Operation {
    Delete(String),
    Set(String, i32),
}

impl FromStr for Operation {
    type Err = Error;
    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        if s.chars().last() == Some('-') {
            return Ok(Operation::Delete(s[..s.len() - 1].to_string()));
        }
        let (label, val) = s.split_once('=').ok_or(Error::ParseError)?;
        Ok(Operation::Set(label.to_string(), val.parse().map_err(|_| Error::ParseError)?))
    }
}

fn hash(s: &str) -> usize {
    s.bytes().fold(0, |r, b| ((r + b as usize) * 17) % 256)
}

pub fn task1<S: AsRef<str>>(lines: &[S]) -> Result<usize> {
    Ok(lines.get(0).ok_or(Error::NoData)?.as_ref().split(',').map(hash).sum())
}

pub fn task2<S: AsRef<str>>(lines: &[S]) -> Result<i32> {
    let operations = lines.get(0).ok_or(Error::NoData)?.as_ref().split(',').map(|s| s.parse()).collect::<Result<Vec<Operation>>>()?;
    let mut boxes: Vec<Vec<(String, i32)>> = vec![Vec::new(); 256];
    for op in operations {
        match op {
            Operation::Delete(label) => {
                let idx = hash(&label);
                if let Some(pos) = boxes[idx].iter().position(|(l, _)| l == &label) {
                    boxes[idx].remove(pos);
                }
            },
            Operation::Set(label, val) => {
                let idx = hash(&label);
                if let Some(pos) = boxes[idx].iter().position(|(l, _)| l == &label) {
                    boxes[idx][pos].1 = val;
                } else {
                    boxes[idx].push((label, val));
                }
            }
        }
    }
    let mut result = 0;
    for (i, row) in boxes.iter().enumerate() {
        for (j, &(_, val)) in row.iter().enumerate() {
            result += (i as i32 + 1) * (j as i32 + 1) * val;
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str =
"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_task1() {
        let lines = DATA.lines().collect::<Vec<_>>();
        assert_eq!(Ok(1320), task1(&lines));
    }

    #[test]
    fn test_task2() {
        let lines = DATA.lines().collect::<Vec<_>>();
        assert_eq!(Ok(145), task2(&lines));
    }

    #[test]
    fn test_hash() {
        assert_eq!(52, hash("HASH"));
    }
}
