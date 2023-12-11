#[derive(thiserror::Error, Debug, Clone, Eq, PartialEq)]
pub enum Error {
    #[error("Parse error")]
    ParseError,
}

pub type Result<T> = std::result::Result<T, Error>;

fn distance(map: &Vec<Vec<usize>>, start: (usize, usize), end: (usize, usize)) -> usize {
    let mut d = 0;
    let start_i = start.0.min(end.0);
    let start_j = start.1.min(end.1);
    let end_i = start.0.max(end.0);
    let end_j = start.1.max(end.1);
    for i in start_i+1..=end_i {
        d += map[i][start_j].max(1);
    }
    for j in start_j+1..=end_j {
        d += map[start_i][j].max(1);
    }
    d
}

fn calc_distances(map: &Vec<Vec<usize>>, start: (usize, usize), n: usize, m: usize) -> usize {
    let mut j = start.1 + 1;
    let mut total = 0;
    for i in start.0..n {
        while j < m {
            if map[i][j] == 0 {
                total += distance(map, start, (i, j));
            }
            j += 1;
        }
        j = 0;
    }
    total
}

fn expand<S: AsRef<str>>(lines: &[S], multiplier: usize) -> Result<usize> {
    let mut map = lines.iter()
    .map(|s| s.as_ref().chars()
        .map(|c| match c {
            '#' => Ok(0usize),
            '.' => Ok(1usize),
            _ => Err(Error::ParseError)
        }).collect::<Result<Vec<_>>>()
    ).collect::<Result<Vec<_>>>()?;
    let n = map.len();
    let m = map[0].len();
    for i in 0..n {
        if !map[i].iter().any(|x| *x == 0) {
            map[i].iter_mut().for_each(|x| *x = multiplier);
        }
    }
    for j in 0..m {
        if !map.iter().any(|row| row[j] == 0) {
            map.iter_mut().for_each(|row| row[j] = multiplier);
        }
    }
    let mut total = 0;
    for i in 0..n {
        for j in 0..m {
            if map[i][j] == 0 {
                total += calc_distances(&map, (i, j), n, m);
            }
        }
    }
    Ok(total)
}

pub fn task1<S: AsRef<str>>(lines: &[S]) -> Result<usize> {
    expand(lines, 2)
}

pub fn task2<S: AsRef<str>>(lines: &[S]) -> Result<usize> {
    expand(lines, 1000000)
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str =
"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_task1() {
        let lines = DATA.lines().collect::<Vec<_>>();
        assert_eq!(Ok(374), task1(&lines));
    }

    #[test]
    fn test_expand() {
        let lines = DATA.lines().collect::<Vec<_>>();
        assert_eq!(Ok(1030), expand(&lines, 10));
        assert_eq!(Ok(8410), expand(&lines, 100));
    }
}
