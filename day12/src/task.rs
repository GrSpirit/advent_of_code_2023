use std::collections::HashMap;

#[derive(thiserror::Error, Debug, Clone, Eq, PartialEq)]
pub enum Error {
    #[error("Parse error")]
    ParseError,
    #[error("No arrangemet")]
    NoArrangement,
}

pub type Result<T> = std::result::Result<T, Error>;

fn parse_line(line: &str, multiplier: usize) -> Result<(Vec<u8>, Vec<u32>)> {
    let (springs, nums) = line.split_once(' ').ok_or(Error::ParseError)?;
    if springs.contains(|c| !(c == '.' || c == '#' || c == '?')) {
        return Err(Error::ParseError);
    }
    if nums.contains(|c: char| !(c.is_ascii_digit() || c == ',')) {
        return  Err(Error::ParseError);
    }
    let multi_springs = vec![springs; multiplier];
    let multi_nums = vec![nums; multiplier];
    Ok((multi_springs.join("?").into_bytes(), multi_nums.into_iter().map(|s| s.split(',').map(|n| n.parse().unwrap())).flatten().collect()))
}

fn arrangement(mut springs: Vec<u8>, mut nums: Vec<u32>, cache: &mut HashMap<(Vec<u8>, Vec<u32>), Option<u64>>) -> Option<u64> {
    if let Some(res) = cache.get(&(springs.clone(), nums.clone())) {
        return *res;
    }
    let mut i = springs.len();
    while i > 0 {
        i -= 1;
        match springs[i] {
            b'#' => {
                *nums.last_mut()? -= 1;
                if nums.last()? == &0{
                    if i > 0 && springs[i - 1] == b'#' {

                        return None;
                    }
                    if i > 0 && springs[i - 1] == b'?' {
                        springs[i - 1] = b'.';
                    }
                    nums.pop();
                } else {
                    if i > 0 && springs[i - 1] == b'.' {
                        return None;
                    }
                    if i > 0 && springs[i - 1] == b'?' {
                        springs[i - 1] = b'#';
                    }
                }
            },
            b'?' => {
                let mut clone_springs = springs[0..i+1].to_owned();
                *clone_springs.last_mut()? = b'#';
                let res1 = arrangement(clone_springs.clone(), nums.clone(), cache);
                cache.insert((clone_springs.clone(), nums.clone()), res1.clone());

                *clone_springs.last_mut()? = b'.';
                let res2 = arrangement(clone_springs.clone(), nums.clone(), cache);
                cache.insert((clone_springs.clone(), nums.clone()), res2.clone());
                return Some(res1.unwrap_or(0) + res2.unwrap_or(0));
            },
            b'.' => {},
            _ => unreachable!()
        }
    }
    if !nums.is_empty() {
        return None;
    }
    Some(1)
}

pub fn task1<S: AsRef<str>>(lines: &[S]) -> Result<u64> {
    let mut result = 0;
    for (springs, nums) in lines.iter().map(|s| parse_line(s.as_ref(), 1).unwrap()) {
        result += arrangement(springs, nums, &mut HashMap::new()).ok_or(Error::NoArrangement)?;
    }
    Ok(result)
}

pub fn task2<S: AsRef<str>>(lines: &[S]) -> Result<u64> {
    let mut result = 0;
    for (springs, nums) in lines.iter().map(|s| parse_line(s.as_ref(), 5).unwrap()) {
        result += arrangement(springs, nums, &mut HashMap::new()).ok_or(Error::NoArrangement)?;
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str =
"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn test_task1() {
        let lines = DATA.lines().collect::<Vec<_>>();
        assert_eq!(Ok(21), task1(&lines));
    }

    #[test]
    fn test_task2() {
        let lines = DATA.lines().collect::<Vec<_>>();
        assert_eq!(Ok(525152), task2(&lines));
    }

    #[test]
    fn test_arrangement() {
        let (s, n) = parse_line("?###???????? 3,2,1", 1).unwrap();
        assert_eq!(Some(10), arrangement(s, n, &mut HashMap::new()));
    }
}
