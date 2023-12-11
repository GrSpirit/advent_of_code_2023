#[derive(thiserror::Error, Debug, Clone, Eq, PartialEq)]
pub enum Error {
    #[error("Parse error")]
    ParseError,
}

pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    G,
    UD,
    UL,
    UR,
    DL,
    DR,
    LR,
    S
}

impl TryFrom<char> for Dir {
    type Error = Error;
    fn try_from(value: char) -> std::prelude::v1::Result<Self, Self::Error> {
        match value {
            '.' => Ok(Dir::G),
            '|' => Ok(Dir::UD),
            '-' => Ok(Dir::LR),
            'L' => Ok(Dir::UR),
            'J' => Ok(Dir::UL),
            '7' => Ok(Dir::DL),
            'F' => Ok(Dir::DR),
            'S' => Ok(Dir::S),
            _ => Err(Error::ParseError)
        }
    }
}

fn find_start(grid: &Vec<Vec<Dir>>) -> (usize, usize) {
    let i = grid.iter().position(|v| v.contains(&Dir::S)).unwrap();
    let j = grid[i].iter().position(|d| d == &Dir::S).unwrap();
    return (i, j);
}

fn loop_length(map: &mut Vec<Vec<u8>>, grid: &Vec<Vec<Dir>>, start: (usize, usize)) -> i32 {
    // for line in map.iter() {
    //     println!("{}", std::str::from_utf8(line).unwrap());
    // }
    // println!("-----------------");

    let mut len = 0;
    let (mut from, mut pos) = ((0, 0), start);
    loop {
        map[pos.0][pos.1] = b'S';
        len += 1;
        (from, pos) = match grid[pos.0][pos.1] {
            Dir::UD => if from.0 + 1 == pos.0 {
                (pos, (pos.0 + 1, pos.1))
            } else {
                (pos, (pos.0 - 1, pos.1))
            },
            Dir::LR => if from.1 + 1 == pos.1 {
                (pos, (pos.0, pos.1 + 1))
            } else {
                (pos, (pos.0, pos.1 - 1))
            },
            Dir::UR => if (from.0 + 1, from.1) == pos {
                (pos, (pos.0, pos.1 + 1))
            } else {
                (pos, (pos.0 - 1, pos.1))
            },
            Dir::UL => if (from.0 + 1, from.1) == pos {
                (pos, (pos.0, pos.1 - 1))
            } else {
                (pos, (pos.0 - 1, pos.1))
            },
            Dir::DL => if (from.0, from.1 + 1) == pos {
                (pos, (pos.0 + 1, pos.1))
            } else {
                (pos, (pos.0, pos.1 - 1))
            },
            Dir::DR => if (pos.0, pos.1 + 1) == from {
                (pos, (pos.0 + 1, pos.1))
            } else {
                (pos, (pos.0, pos.1 + 1))
            },
            Dir::G => panic!("Cannot be here"),
            Dir::S => if from == (0, 0) {
                (pos, (pos.0, pos.1 + 1))
            } else {
                break;
            }
        }
    }
    return len;
}

pub fn task1<S: AsRef<str>>(lines: &[S]) -> Result<i32> {
    let grid = lines.iter().map(|s| s.as_ref().chars().map(|c| c.try_into().unwrap()).collect::<Vec<Dir>>()).collect::<Vec<_>>();
    let mut map = lines.iter().map(|s| s.as_ref().as_bytes().to_vec()).collect::<Vec<_>>();
    Ok(loop_length(&mut map, &grid, find_start(&grid)) / 2)
}

pub fn task2<S: AsRef<str>>(lines: &[S]) -> Result<usize> {
    let grid = lines.iter().map(|s| s.as_ref().chars().map(|c| c.try_into().unwrap()).collect::<Vec<Dir>>()).collect::<Vec<_>>();
    let mut map = lines.iter().map(|s| s.as_ref().as_bytes().to_vec()).collect::<Vec<_>>();
    loop_length(&mut map, &grid, find_start(&grid));
    for line in &mut map {
        for x in line {
            if *x != b'S' {
                *x = b'.';
            }
        }
    }
    for i in 0..grid.len() {
        let mut s = b'O';
        for j in 0..grid[i].len() {
            if map[i][j] == b'.' {
                map[i][j] = s;
            } else {
                if grid[i][j] != Dir::LR {
                    s = if s == b'O' { b'I' } else { b'O' };
                }
            }
        }
        println!("{}", std::str::from_utf8(&map[i]).unwrap());
    }

    Ok(map.iter().map(|l| l.iter().filter(|b| **b == b'I').count()).sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA1: &str =
"..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    const DATA2: &str =
".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

    #[test]
    fn test_task1() {
        let lines = DATA1.lines().collect::<Vec<_>>();
        assert_eq!(Ok(8), task1(&lines));
    }

    #[test]
    fn test_task2() {
        let lines = DATA2.lines().collect::<Vec<_>>();
        assert_eq!(Ok(8), task2(&lines));
    }
}
