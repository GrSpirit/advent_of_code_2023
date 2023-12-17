use std::collections::VecDeque;

#[derive(thiserror::Error, Debug, Clone, Eq, PartialEq)]
pub enum Error {
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    Up = 1,
    Down = 2,
    Left = 4,
    Right = 8,
}

impl From<Dir> for u8 {
    fn from(dir: Dir) -> Self {
        dir as u8
    }
}

struct Beam {
    pos: (usize, usize),
    dir: Dir,
}

fn compute(grid: &[&[u8]], start_beam: Beam) -> usize {
    let n = grid.len();
    let m = grid[0].len();
    let mut visited: Vec<Vec<u8>> = vec![vec![0; m]; n];
    let mut q = VecDeque::new();
    q.push_back(start_beam);
    while let Some(Beam{pos: (i, j), dir}) = q.pop_front() {
        if (visited[i][j] & u8::from(dir)) != 0 {
            continue;
        }
        visited[i][j] |= dir as u8;
        match grid[i][j] {
            b'.' => {
                let next_pos = match dir {
                    Dir::Up => {
                        if i == 0 { continue; }
                        (i - 1, j)
                    },
                    Dir::Down => {
                        if i + 1 == n { continue; }
                        (i + 1, j)
                    },
                    Dir::Left => {
                        if j == 0 { continue; }
                        (i, j - 1)
                    },
                    Dir::Right => {
                        if j + 1 == m { continue; }
                        (i, j + 1)
                    }
                };
                q.push_back(Beam{ pos: next_pos, dir });
            },
            b'-' => {
                match dir {
                    Dir::Left => {
                        if j == 0 { continue; }
                        q.push_back(Beam{ pos: (i, j - 1), dir });
                    },
                    Dir::Right => {
                        if j + 1 == m { continue; }
                        q.push_back(Beam{ pos: (i, j + 1), dir });
                    },
                    Dir::Up | Dir::Down => {
                        if j > 0 {
                            q.push_back(Beam { pos: (i, j - 1), dir: Dir::Left });
                        }
                        if j + 1 < m {
                            q.push_back(Beam { pos: (i, j + 1), dir: Dir::Right });
                        }
                    }
                }
            },
            b'|' => {
                match dir {
                    Dir::Up => {
                        if i == 0 { continue; }
                        q.push_back(Beam{ pos: (i - 1, j), dir });
                    },
                    Dir::Down => {
                        if i + 1 == n { continue; }
                        q.push_back(Beam{ pos: (i + 1, j), dir });
                    },
                    Dir::Left | Dir::Right => {
                        if i > 0 {
                            q.push_back(Beam { pos: (i - 1, j), dir: Dir::Up });
                        }
                        if i + 1 < n {
                            q.push_back(Beam { pos: (i + 1, j), dir: Dir::Down });
                        }
                    }
                }
            },
            b'\\' => {
                match dir {
                    Dir::Right => {
                        if i + 1 == n { continue; }
                        q.push_back(Beam { pos: (i + 1, j), dir: Dir::Down });
                    },
                    Dir::Left => {
                        if i == 0 { continue; }
                        q.push_back(Beam { pos: (i - 1, j), dir: Dir::Up });
                    },
                    Dir::Up => {
                        if j == 0 { continue; }
                        q.push_back(Beam { pos: (i, j - 1), dir: Dir::Left });
                    },
                    Dir::Down => {
                        if j + 1 == m { continue; }
                        q.push_back(Beam { pos: (i, j + 1), dir: Dir::Right });
                    }
                }
            },
            b'/' => {
                match dir {
                    Dir::Right => {
                        if i == 0 { continue; }
                        q.push_back(Beam { pos: (i - 1, j), dir: Dir::Up })
                    },
                    Dir::Left => {
                        if i + 1 == n { continue; }
                        q.push_back(Beam { pos: (i + 1, j), dir: Dir::Down });
                    },
                    Dir::Up => {
                        if j + 1 == m { continue; }
                        q.push_back(Beam { pos: (i, j + 1), dir: Dir::Right });
                    },
                    Dir::Down => {
                        if j == 0 { continue; }
                        q.push_back(Beam { pos: (i, j - 1), dir: Dir::Left });
                    }
                }
            },
            _ => unreachable!()
        }
    }
    visited.iter().map(|row| row.iter().filter(|b| **b != 0).count()).sum()
}

pub fn task1<S: AsRef<str>>(lines: &[S]) -> Result<usize> {
    let grid = lines.iter().map(|s| s.as_ref().as_bytes()).collect::<Vec<_>>();
    Ok(compute(&grid, Beam{ pos: (0, 0), dir: Dir::Right }))
}
pub fn task2<S: AsRef<str>>(lines: &[S]) -> Result<usize> {
    let grid = lines.iter().map(|s| s.as_ref().as_bytes()).collect::<Vec<_>>();
    let n = grid.len();
    let m = grid[0].len();
    let mut max_score = 0;
    for i in 0..n {
        max_score = max_score.max(compute(&grid, Beam { pos: (i, 0), dir: Dir::Right }));
        max_score = max_score.max(compute(&grid, Beam { pos: (i, m - 1), dir: Dir::Left }));
    }

    for j in 0..m {
        max_score = max_score.max(compute(&grid, Beam { pos: (0, j), dir: Dir::Down }));
        max_score = max_score.max(compute(&grid, Beam { pos: (n - 1, j), dir: Dir::Up }));
    }

    Ok(max_score)
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str =
r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

    #[test]
    fn test_task1() {
        let lines = DATA.lines().collect::<Vec<_>>();
        assert_eq!(Ok(46), task1(&lines));
    }

    #[test]
    fn test_task2() {
        let lines = DATA.lines().collect::<Vec<_>>();
        assert_eq!(Ok(51), task2(&lines));
    }
}
