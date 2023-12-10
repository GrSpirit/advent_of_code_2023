#[derive(thiserror::Error, Debug, Clone, Eq, PartialEq)]
pub enum Error {
    #[error("Wrong node")]
    WrongNode,
    #[error("Wrong instruction")]
    WrongInstruction
}

pub type Result<T> = std::result::Result<T, Error>;

fn extrapolate(v: &[i32]) -> i32 {
    if v.iter().all(|x| *x == 0) {
        return 0;
    }
    let w = v.windows(2).map(|v| v[1] - v[0]).collect::<Vec<i32>>();
    let diff = extrapolate(&w);
    v.last().unwrap() + diff
}

fn extrapolate_backward(v: &[i32]) -> i32 {
    if v.iter().all(|x| *x == 0) {
        return 0;
    }
    let w = v.windows(2).map(|v| v[1] - v[0]).collect::<Vec<i32>>();
    let diff = extrapolate_backward(&w);
    v.first().unwrap() - diff
}

pub fn task1<S: AsRef<str>>(lines: &[S]) -> Result<i32> {
    Ok(lines.iter().map(|l| {
        let v = l.as_ref().split_ascii_whitespace().map(|n| n.parse().unwrap()).collect::<Vec<_>>();
        extrapolate(&v)
    }).sum())
}

pub fn task2<S: AsRef<str>>(lines: &[S]) -> Result<i32> {
    Ok(lines.iter().map(|l| {
        let v = l.as_ref().split_ascii_whitespace().map(|n| n.parse().unwrap()).collect::<Vec<_>>();
        extrapolate_backward(&v)
    }).sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str =
"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_task1() {
        let lines = DATA.lines().collect::<Vec<_>>();
        assert_eq!(Ok(114), task1(&lines));
    }

    #[test]
    fn test_task2() {
        let lines = DATA.lines().collect::<Vec<_>>();
        assert_eq!(Ok(2), task2(&lines));
    }
}
