use std::{str::Utf8Error, num::ParseIntError, collections::HashMap};

#[derive(thiserror::Error, Debug, Clone, Eq, PartialEq)]
pub enum Error {
    #[error("Parse utf8 error")]
    ParseUtf8Error(#[from] Utf8Error),
    #[error("Parse int error")]
    ParseIntError(#[from] ParseIntError),
}

pub type Result<T> = std::result::Result<T, Error>;


pub fn task1<S: AsRef<str>>(lines: &[S]) -> Result<u32> {
    let mut result = 0;
    let grid = lines.into_iter().map(|s| s.as_ref().as_bytes()).collect::<Vec<_>>();
    let (n, m) = (grid.len(), grid[0].len());
    let is_symbol = |i: usize, j: usize| -> bool {
        !grid[i][j].is_ascii_digit() && grid[i][j] != b'.'
    };
    let check_around = |ii: usize, jj: usize| -> bool {
        for i in (ii.checked_sub(1).unwrap_or(0))..(ii + 2).min(n) {
            for j in (jj.checked_sub(1).unwrap_or(0))..(jj + 2).min(m) {
                if i == ii && j == jj {
                    continue;
                }
                if is_symbol(i, j) {
                    return true;
                }
            }
        }
        return false;
    };
    for i in 0..n {
        let mut j = 0;
        while j < m {
            if grid[i][j].is_ascii_digit() {
                let mut jlast= j;
                let mut found_symbol = false;
                while jlast < m && grid[i][jlast].is_ascii_digit() {
                    found_symbol |= check_around(i, jlast);
                    jlast += 1;
                }
                if found_symbol {
                    result += std::str::from_utf8(&grid[i][j..jlast])?.parse::<u32>()?;
                }
                j = jlast;
            } else {
                j += 1;
            }
        }
    }
    Ok(result)
}

pub fn task2<S: AsRef<str>>(lines: &[S]) -> Result<u32> {
    let mut all_stars: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    let grid = lines.into_iter().map(|s| s.as_ref().as_bytes()).collect::<Vec<_>>();
    let (n, m) = (grid.len(), grid[0].len());
    let is_symbol = |i: usize, j: usize| -> bool {
        grid[i][j] == b'*'
    };
    let check_around = |ii: usize, jj: usize| -> Vec<(usize, usize)> {
        let mut stars = Vec::new();
        for i in (ii.checked_sub(1).unwrap_or(0))..(ii + 2).min(n) {
            for j in (jj.checked_sub(1).unwrap_or(0))..(jj + 2).min(m) {
                if i == ii && j == jj {
                    continue;
                }
                if is_symbol(i, j) {
                    stars.push((i, j));
                }
            }
        }
        return stars;
    };
    for i in 0..n {
        let mut j = 0;
        while j < m {
            if grid[i][j].is_ascii_digit() {
                let mut jlast= j;
                let mut stars = Vec::new();
                while jlast < m && grid[i][jlast].is_ascii_digit() {
                    stars.extend_from_slice(&check_around(i, jlast));
                    jlast += 1;
                }
                stars.sort();
                stars.dedup();
                for star in stars {
                    all_stars.entry(star).or_default().push(std::str::from_utf8(&grid[i][j..jlast])?.parse::<u32>()?);
                }
                j = jlast;
            } else {
                j += 1;
            }
        }
    }
    Ok(all_stars.into_values().filter(|v| v.len() == 2).map(|v| v[0] * v[1]).sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str = 
"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_task1() {
        let lines = DATA.lines().collect::<Vec<_>>();
        assert_eq!(Ok(4361), task1(&lines));
    }

    #[test]
    fn test_task2() {
        let lines = DATA.lines().collect::<Vec<_>>();
        assert_eq!(Ok(467835), task2(&lines));
    }

}
