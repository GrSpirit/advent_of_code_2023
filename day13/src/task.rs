#[derive(thiserror::Error, Debug, Clone, Eq, PartialEq)]
pub enum Error {
    #[error("No pattern found")]
    NoPatternFound,
}

pub type Result<T> = std::result::Result<T, Error>;

fn mirror_vertical(grid: &[&[u8]], mut l: usize, mut r: usize, mismatch_limit: u32) -> bool {
    let mut mismatched = 0;
    while l < r {
        for i in 0..grid.len() {
            if grid[i][l] != grid[i][r] {
                mismatched += 1;
                if mismatched > mismatch_limit {
                    return false;
                }
            }
        }
        l += 1;
        r -= 1;
    }
    return mismatched == mismatch_limit;
}

fn mirror_horizontal(grid: &[&[u8]], mut l: usize, mut r: usize, mismatch_limit: u32) -> bool {
    let mut mismatched = 0;
    while l < r {
        for i in 0..grid[0].len() {
            if grid[l][i] != grid[r][i] {
                mismatched += 1;
                if mismatched > mismatch_limit {
                    return false;
                }
            }
        }
        l += 1;
        r -= 1;
    }
    return mismatched == mismatch_limit;
}

fn find_mirror<F: Fn(&[&[u8]], usize, usize, u32) -> bool>(grid: &[&[u8]], n: usize, mismatch_limit: u32, f: F) -> Option<usize> {
    (1..n).find(|&i| {
        let d = usize::min(i, n - i);
        let l = i - d;
        let r = i + d - 1;
        f(grid, l, r, mismatch_limit)
    })
}

pub fn solve<S: AsRef<str>>(lines: &[S], mismatch_limit: u32) -> Result<usize> {
    let mut result = 0;
    for pattern in lines.split(|s| s.as_ref().is_empty()) {
        let grid = pattern.iter().map(|s| s.as_ref().as_bytes()).collect::<Vec<_>>();
        result += find_mirror(&grid, grid[0].len(), mismatch_limit, mirror_vertical)
        .or_else(|| find_mirror(&grid, grid.len(), mismatch_limit, mirror_horizontal).map(|x| x * 100)).ok_or(Error::NoPatternFound)?;
    }
    Ok(result)
}

pub fn task1<S: AsRef<str>>(lines: &[S]) -> Result<usize> {
    solve(lines, 0)
}

pub fn task2<S: AsRef<str>>(lines: &[S]) -> Result<usize> {
    solve(lines, 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str =
"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn test_task1() {
        let lines = DATA.lines().collect::<Vec<_>>();
        assert_eq!(Ok(405), task1(&lines));
    }

    #[test]
    fn test_task2() {
        let lines = DATA.lines().collect::<Vec<_>>();
        assert_eq!(Ok(400), task2(&lines));
    }
}
