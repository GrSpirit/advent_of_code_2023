use std::cmp::Reverse;
use std::collections::{HashSet, BinaryHeap};
use std::ops::{Add, Mul, Sub};

#[derive(thiserror::Error, Debug, Clone, Eq, PartialEq)]
pub enum Error {
}

pub type Result<T> = std::result::Result<T, Error>;

fn to_i(b: u8) -> u32 { (b - b'0') as u32 }

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point(isize, isize);

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self( self.0 + rhs.0, self.1 + rhs.1 )
    }
}

impl Mul<isize> for Point {
    type Output = Point;
    fn mul(self, rhs: isize) -> Self::Output {
        Self ( self.0 * rhs, self.1 * rhs )
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self ( self.0 - rhs.0, self.1 - rhs.1 )
    }
}

fn dijkstra(grid: &Vec<Vec<u32>>) -> u32 {
    let n = grid.len() as isize;
    let m = grid[0].len() as isize;
    let is_valid_pos = |p: &Point| -> bool {
        p.0 >= 0 && p.1 >= 0 && p.0 < n && p.1 < m
    };

    let mut pq = BinaryHeap::new();
    pq.push((Reverse(0), Point(0, 0), Point(0, 0), 0));
    let mut visited: HashSet<(Point, Point, i32)> = HashSet::new();
    while let Some((Reverse(score), pos, dir, len)) = pq.pop() {
        if pos == Point(n - 1, m - 1) {
            return score;
        }
        if !visited.insert((pos, dir, len)) {
            continue;
        }
        for d in [Point(-1, 0), Point(1, 0), Point(0, -1), Point(0, 1)] {
            let next_pos = pos + d;
            if !is_valid_pos(&next_pos) { continue; }
            if next_pos + dir == pos { continue; }
            if d == dir && len == 3 { continue; }
            pq.push((Reverse(score + grid[next_pos.0 as usize][next_pos.1 as usize] as u32), next_pos, d, if d == dir { len + 1 } else { 1 }));
        }
    }
    return 0;
}

fn dijkstra2(grid: &Vec<Vec<u32>>) -> u32 {
    let n = grid.len() as isize;
    let m = grid[0].len() as isize;
    let is_valid_pos = |p: &Point| -> bool {
        p.0 >= 0 && p.1 >= 0 && p.0 < n && p.1 < m
    };

    let mut pq = BinaryHeap::new();
    pq.push((Reverse(0), Point(0, 0), Point(0, 1), 0));
    pq.push((Reverse(0), Point(0, 0), Point(1, 0), 0));
    let mut visited: HashSet<(Point, Point, i32)> = HashSet::new();
    while let Some((Reverse(score), pos, dir, len)) = pq.pop() {
        if pos == Point(n - 1, m - 1) {
            return score;
        }
        if !visited.insert((pos, dir, len)) {
            continue;
        }
        for d in [Point(-1, 0), Point(1, 0), Point(0, -1), Point(0, 1)] {
            let next_pos = pos + d;
            if !is_valid_pos(&next_pos) { continue; }
            if next_pos + dir == pos { continue; }
            if d != dir && len < 4 { continue; }
            if d == dir && len >= 10 { continue; }
            pq.push((Reverse(score + grid[next_pos.0 as usize][next_pos.1 as usize] as u32), next_pos, d, if d == dir { len + 1 } else { 1 }));
        }
    }
    return 0;
}

pub fn task1<S: AsRef<str>>(lines: &[S]) -> Result<u32> {
    let grid = lines.iter().map(|s| s.as_ref().bytes().map(to_i).collect::<Vec<_>>()).collect::<Vec<_>>();
    Ok(dijkstra(&grid))
}

pub fn task2<S: AsRef<str>>(lines: &[S]) -> Result<u32> {
    let grid = lines.iter().map(|s| s.as_ref().bytes().map(to_i).collect::<Vec<_>>()).collect::<Vec<_>>();
    Ok(dijkstra2(&grid))
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str =
r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#;

    #[test]
    fn test_task1() {
        let lines = DATA.lines().collect::<Vec<_>>();
        assert_eq!(Ok(102), task1(&lines));
    }

    #[test]
    fn test_task2() {
        let lines = DATA.lines().collect::<Vec<_>>();
        assert_eq!(Ok(94), task2(&lines));
    }
}
