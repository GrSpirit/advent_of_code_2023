use std::collections::HashMap;
use gcd::Gcd;

#[derive(thiserror::Error, Debug, Clone, Eq, PartialEq)]
pub enum Error {
    #[error("Wrong node")]
    WrongNode,
    #[error("Wrong instruction")]
    WrongInstruction
}

pub type Result<T> = std::result::Result<T, Error>;

fn parse_map<S: AsRef<str>>(lines: &[S]) -> HashMap<String, (String, String)> {
    lines.into_iter().map(|s| {
        let s = s.as_ref();
        (s[0..3].to_string(), (s[7..10].to_string(), s[12..15].to_string()))
    }).collect()
}

pub fn task1<S: AsRef<str>>(lines: &[S]) -> Result<u32> {
    let instructions = lines[0].as_ref();
    let map = parse_map(&lines[2..]);
    let mut current = "AAA";
    let mut count = 0;
    for dir in instructions.chars().cycle() {
        current = match dir {
            'L' => &map.get(current).ok_or(Error::WrongNode)?.0,
            'R' => &map.get(current).ok_or(Error::WrongNode)?.1,
            _ => return Err(Error::WrongInstruction),
        };
        count += 1;
        if current == "ZZZ" {
            break;
        }
    }
    Ok(count)
}

fn find_cycle(start: &str, instructions: &str, map: &HashMap<String, (String, String)>) -> Result<u64> {
    let mut key = start;
    let mut count = 0;
    for dir in instructions.chars().cycle() {
        key = match dir {
            'L' => &map.get(key).ok_or(Error::WrongNode)?.0,
            'R' => &map.get(key).ok_or(Error::WrongNode)?.1,
            _ => return Err(Error::WrongInstruction),
        };
        count += 1;
        if key.ends_with('Z') {
            return Ok(count);
        }
    }
    unreachable!()
}

pub fn task2<S: AsRef<str>>(lines: &[S]) -> Result<u64> {
    let instructions = lines[0].as_ref();
    let map = parse_map(&lines[2..]);
    let current = map.keys().filter(|s| s.ends_with('A')).collect::<Vec<_>>();
    let results = current.iter().map(|cc| find_cycle(cc, instructions, &map)).collect::<Result<Vec<u64>>>()?;
    Ok(results.into_iter().reduce(|a, b| a / a.gcd(b) * b).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA1: &str =
"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const DATA2: &str =
"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test_task1() {
        let lines = DATA1.lines().collect::<Vec<_>>();
        assert_eq!(Ok(6), task1(&lines));
    }

    #[test]
    fn test_task2() {
        let lines = DATA2.lines().collect::<Vec<_>>();
        assert_eq!(Ok(6), task2(&lines));
    }
}
