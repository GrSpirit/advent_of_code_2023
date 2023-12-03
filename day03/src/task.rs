use std::{str::Utf8Error, num::ParseIntError, collections::{HashMap, HashSet}};

#[derive(thiserror::Error, Debug, Clone, Eq, PartialEq)]
pub enum Error {
    #[error("Parse utf8 error")]
    ParseUtf8Error(#[from] Utf8Error),
    #[error("Parse int error")]
    ParseIntError(#[from] ParseIntError),
}

pub type Result<T> = std::result::Result<T, Error>;

trait Checker {
    fn check_around(&mut self, ii: usize, jj: usize);
    fn apply(&mut self, value: u32);
    fn result(&self) -> u32;
}

struct Task1Checker<'a> {
    grid: &'a [&'a [u8]],
    acc: u32,
    found: bool,
}

impl<'a> Task1Checker<'a> {
    fn new(grid: &'a [&'a [u8]]) -> Self {
        Self { grid, acc: 0, found: false }
    }
    fn is_symbol(&self, i: usize, j: usize) -> bool {
        !self.grid[i][j].is_ascii_digit() && self.grid[i][j] != b'.'
    }
}

impl<'a> Checker for Task1Checker<'a> {
    fn check_around(&mut self, ii: usize, jj: usize) {
        let n = self.grid.len();
        let m = self.grid[0].len();
        for i in (ii.checked_sub(1).unwrap_or(0))..(ii + 2).min(n) {
            for j in (jj.checked_sub(1).unwrap_or(0))..(jj + 2).min(m) {
                if i == ii && j == jj {
                    continue;
                }
                if self.is_symbol(i, j) {
                    self.found = true;
                    return;
                }
            }
        }
    }
    fn apply(&mut self, value: u32) {
        if self.found {
            self.acc += value;
        }
        self.found = false;
    }
    fn result(&self) -> u32 {
        self.acc
    }
}


struct Task2Checker<'a> {
    grid: &'a [&'a [u8]],
    all_stars: HashMap<(usize, usize), Vec<u32>>,
    tmp_stars: HashSet<(usize, usize)>,
}

impl<'a> Task2Checker<'a> {
    fn new(grid: &'a [&'a [u8]]) -> Self {
        Self { grid, all_stars: HashMap::new(), tmp_stars: HashSet::new()}
    }
    fn is_symbol(&self, i: usize, j: usize) -> bool {
        self.grid[i][j] == b'*'
    }
}

impl<'a> Checker for Task2Checker<'a> {
    fn check_around(&mut self, ii: usize, jj: usize) {
        let n = self.grid.len();
        let m = self.grid[0].len();
        for i in (ii.checked_sub(1).unwrap_or(0))..(ii + 2).min(n) {
            for j in (jj.checked_sub(1).unwrap_or(0))..(jj + 2).min(m) {
                if i == ii && j == jj {
                    continue;
                }
                if self.is_symbol(i, j) {
                    self.tmp_stars.insert((i, j));
                }
            }
        }
    }
    fn apply(&mut self, value: u32) {
        for star in &self.tmp_stars {
            self.all_stars.entry(*star).or_default().push(value);
        }
        self.tmp_stars.clear();
    }
    fn result(&self) -> u32 {
        self.all_stars.values().filter(|v| v.len() == 2).map(|v| v[0] * v[1]).sum()
    }
}

fn process<C: Checker>(grid: &[&[u8]], mut checker: C) -> Result<u32> {
    let (n, m) = (grid.len(), grid[0].len());
    for i in 0..n {
        let mut j = 0;
        while j < m {
            if grid[i][j].is_ascii_digit() {
                let mut jlast= j;
                while jlast < m && grid[i][jlast].is_ascii_digit() {
                    checker.check_around(i, jlast);
                    jlast += 1;
                }
                checker.apply(std::str::from_utf8(&grid[i][j..jlast])?.parse::<u32>()?);
                j = jlast;
            } else {
                j += 1;
            }
        }
    }
    Ok(checker.result())
}

pub fn task1<S: AsRef<str>>(lines: &[S]) -> Result<u32> {
    let grid = lines.into_iter().map(|s| s.as_ref().as_bytes()).collect::<Vec<_>>();
    process(&grid, Task1Checker::new(&grid))
}

pub fn task2<S: AsRef<str>>(lines: &[S]) -> Result<u32> {
    let grid = lines.into_iter().map(|s| s.as_ref().as_bytes()).collect::<Vec<_>>();
    process(&grid, Task2Checker::new(&grid))
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
