#[derive(thiserror::Error, Debug, Clone, Eq, PartialEq)]
pub enum Error {
    #[error("No pattern found")]
    NoPatternFound,
}

pub type Result<T> = std::result::Result<T, Error>;

fn mirror_vertical(grid: &[&[u8]], pos: usize, mismatch_limit: u32) -> bool {
    let d = usize::min(pos, grid[0].len() - pos);
    let mut l = pos - d;
    let mut r = pos + d - 1;
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

fn mirror_horizontal(grid: &[&[u8]], pos: usize, mismatch_limit: u32) -> bool {
    let d = usize::min(pos, grid.len() - pos);
    let mut l = pos - d;
    let mut r = pos + d - 1;
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

fn find_mirror<F: Fn(&[&[u8]], usize, u32) -> bool>(grid: &[&[u8]], n: usize, mismatch_limit: u32, f: F) -> Option<usize> {
    (1..n).find(|&i| f(grid, i, mismatch_limit))
}

pub fn task1<S: AsRef<str>>(lines: &[S]) -> Result<usize> {
    let mut result = 0;
    for pattern in lines.split(|s| s.as_ref().is_empty()) {
        let grid = pattern.iter().map(|s| s.as_ref().as_bytes()).collect::<Vec<_>>();
        result += find_mirror(&grid, grid[0].len(), 0, mirror_vertical)
        .or_else(|| find_mirror(&grid, grid.len(), 0, mirror_horizontal).map(|x| x * 100)).ok_or(Error::NoPatternFound)?;
    }
    Ok(result)
}

pub fn task2<S: AsRef<str>>(lines: &[S]) -> Result<usize> {
    let mut result = 0;
    for pattern in lines.split(|s| s.as_ref().is_empty()) {
        let grid = pattern.iter().map(|s| s.as_ref().as_bytes()).collect::<Vec<_>>();
        result += find_mirror(&grid, grid[0].len(), 1, mirror_vertical)
        .or_else(|| find_mirror(&grid, grid.len(), 1, mirror_horizontal).map(|x| x * 100)).ok_or(Error::NoPatternFound)?;
    }
    Ok(result)
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
