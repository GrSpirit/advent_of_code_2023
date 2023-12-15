#[derive(thiserror::Error, Debug, Clone, Eq, PartialEq)]
pub enum Error {
    #[error("no cycle")]
    NoCycle,
}

pub type Result<T> = std::result::Result<T, Error>;

fn sort_north(grid: &mut Vec<Vec<u8>>) {
    let n = grid.len();
    let m = grid[0].len();
    for j in 0..m {
        let mut i = 0;
        let mut k = 0;
        while i < n {
            match grid[i][j] {
                b'O' | b'#' => { },
                b'.' => {
                    k = k.max(i + 1);
                    while k < n {
                        match grid[k][j] {
                            b'#' => { break; },
                            b'.' => { k += 1; },
                            b'O' => {
                                grid[k][j] = b'.';
                                grid[i][j] = b'O';
                                k += 1;
                                break;
                            },
                            _ => unreachable!()
                        }
                    }
                    if k == n { break; }
                },
                _ => unreachable!()
            }
            i += 1;
        }
    }
}

fn sort_south(grid: &mut Vec<Vec<u8>>) {
    let n = grid.len();
    let m = grid[0].len();
    for j in 0..m {
        let mut i = n as isize - 1;
        let mut k = i;
        while i >= 0 {
            match grid[i as usize][j] {
                b'O' | b'#' => { },
                b'.' => {
                    k = k.min(i - 1);
                    while k >= 0 {
                        match grid[k as usize][j] {
                            b'#' => { break; },
                            b'.' => { k -= 1; },
                            b'O' => {
                                grid[k as usize][j] = b'.';
                                grid[i as usize][j] = b'O';
                                k -= 1;
                                break;
                            },
                            _ => unreachable!()
                        }
                    }
                    if k < 0 { break; }
                },
                _ => unreachable!()
            }
            i -= 1;
        }
    }
}

fn sort_west(grid: &mut Vec<Vec<u8>>) {
    let n = grid.len();
    let m = grid[0].len();
    for i in 0..n {
        let mut j = 0;
        let mut k = 0;
        while j < m {
            match grid[i][j] {
                b'O' | b'#' => { },
                b'.' => {
                    k = k.max(j + 1);
                    while k < m {
                        match grid[i][k] {
                            b'#' => { break; },
                            b'.' => { k += 1; },
                            b'O' => {
                                grid[i][k] = b'.';
                                grid[i][j] = b'O';
                                k += 1;
                                break;
                            },
                            _ => unreachable!()
                        }
                    }
                    if k == n { break; }
                },
                _ => unreachable!()
            }
            j += 1;
        }
    }
}

fn sort_east(grid: &mut Vec<Vec<u8>>) {
    let n = grid.len();
    let m = grid[0].len();
    for i in 0..n {
        let mut j = m as isize - 1;
        let mut k = j;
        while j >= 0 {
            match grid[i][j as usize] {
                b'O' | b'#' => { },
                b'.' => {
                    k = k.min(j - 1);
                    while k >= 0 {
                        match grid[i][k as usize] {
                            b'#' => { break; },
                            b'.' => { k -= 1; },
                            b'O' => {
                                grid[i][k as usize] = b'.';
                                grid[i][j as usize] = b'O';
                                k -= 1;
                                break;
                            },
                            _ => unreachable!()
                        }
                    }
                    if k < 0 { break; }
                },
                _ => unreachable!()
            }
            j -= 1;
        }
    }
}

fn calc_load(grid: &Vec<Vec<u8>>) -> usize {
    let result = grid.iter().rev().enumerate().map(|(i, row)| {
        row.iter().filter(|x| **x == b'O').count() * (i + 1)
    }).sum();
    result
}

fn rotate(grid: &mut Vec<Vec<u8>>) {
    sort_north(grid);
    sort_west(grid);
    sort_south(grid);
    sort_east(grid);
}

fn cycle_len(nums: &[usize]) -> Option<usize> {
    let n = nums.len();
    for l in 2..n {
        if nums[n - l..n] == nums[n - l * 2..n - l] {
            return Some(l);
        }
    }
    return None;
}

pub fn task1<S: AsRef<str>>(lines: &[S]) -> Result<usize> {
    let mut grid = lines.into_iter().map(|l| l.as_ref().to_string().into_bytes()).collect::<Vec<_>>();
    sort_north(&mut grid);
    let result = calc_load(&grid);
    Ok(result)
}

pub fn task2<S: AsRef<str>>(lines: &[S]) -> Result<usize> {
    let mut grid = lines.into_iter().map(|l| l.as_ref().to_string().into_bytes()).collect::<Vec<_>>();
    let mut nums = Vec::with_capacity(100);
    for _ in 0..100 {
        rotate(&mut grid);
        nums.push(calc_load(&grid));
    }
    println!("{:?}", nums);
    let n = cycle_len(&nums).ok_or(Error::NoCycle)?;
    let pos = (1000000000 - 99) % n;
    Ok(nums[pos])
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str =
"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    const DATA_SORTED_NORTH: &str =
"OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....";

    const DATA_SORTED_SOUTH: &str =
".....#....
....#....#
...O.##...
...#......
O.O....O#O
O.#..O.#.#
O....#....
OO....OO..
#OO..###..
#OO.O#...O";

    const DATA_SORTED_WEST: &str =
"O....#....
OOO.#....#
.....##...
OO.#OO....
OO......#.
O.#O...#.#
O....#OO..
O.........
#....###..
#OO..#....";

    const DATA_SORTED_EAST: &str =
"....O#....
.OOO#....#
.....##...
.OO#....OO
......OO#.
.O#...O#.#
....O#..OO
.........O
#....###..
#..OO#....";

    const DATA_3_cycles: &str =
".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O";

    #[test]
    fn test_task1() {
        let lines = DATA.lines().collect::<Vec<_>>();
        assert_eq!(Ok(136), task1(&lines));
    }

    #[test]
    fn test_task2() {
        let lines = DATA.lines().collect::<Vec<_>>();
        assert_eq!(Ok(64), task2(&lines));
    }

    #[test]
    fn test_sort_north() {
        use itertools::Itertools;
        let lines = DATA.lines().collect::<Vec<_>>();
        let mut grid = lines.into_iter().map(|l| l.to_string().into_bytes()).collect::<Vec<_>>();
        sort_north(&mut grid);
        let res = grid.into_iter().map(|row| String::from_utf8(row).unwrap()).join("\n");
        assert_eq!(DATA_SORTED_NORTH, res);
    }

    #[test]
    fn test_sort_south() {
        use itertools::Itertools;
        let lines = DATA.lines().collect::<Vec<_>>();
        let mut grid = lines.into_iter().map(|l| l.to_string().into_bytes()).collect::<Vec<_>>();
        sort_south(&mut grid);
        let res = grid.into_iter().map(|row| String::from_utf8(row).unwrap()).join("\n");
        assert_eq!(DATA_SORTED_SOUTH, res);
    }

    #[test]
    fn test_sort_west() {
        use itertools::Itertools;
        let lines = DATA.lines().collect::<Vec<_>>();
        let mut grid = lines.into_iter().map(|l| l.to_string().into_bytes()).collect::<Vec<_>>();
        sort_west(&mut grid);
        let res = grid.into_iter().map(|row| String::from_utf8(row).unwrap()).join("\n");
        assert_eq!(DATA_SORTED_WEST, res);
    }

    #[test]
    fn test_sort_east() {
        use itertools::Itertools;
        let lines = DATA.lines().collect::<Vec<_>>();
        let mut grid = lines.into_iter().map(|l| l.to_string().into_bytes()).collect::<Vec<_>>();
        sort_east(&mut grid);
        let res = grid.into_iter().map(|row| String::from_utf8(row).unwrap()).join("\n");
        assert_eq!(DATA_SORTED_EAST, res);
    }

    // #[test]
    // fn test_3_cycles() {
    //     use itertools::Itertools;
    //     let lines = DATA.lines().collect::<Vec<_>>();
    //     let mut grid = lines.into_iter().map(|l| l.to_string().into_bytes()).collect::<Vec<_>>();
    //     rotate(&mut grid);
    //     rotate(&mut grid);
    //     rotate(&mut grid);
    //     assert_eq!(64, calc_load(&grid));
    //     let res = grid.into_iter().map(|row| String::from_utf8(row).unwrap()).join("\n");
    //     assert_eq!(DATA_3_cycles, res);
    // }
}
