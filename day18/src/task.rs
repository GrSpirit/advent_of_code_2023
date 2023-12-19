use std::{str::FromStr, ops::{Mul, AddAssign, Add}};

#[derive(thiserror::Error, Debug, Clone, Eq, PartialEq)]
pub enum Error {
    #[error("Format error")]
    FormatError,
    #[error("Parse int error")]
    ParseIntError(#[from] std::num::ParseIntError),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    Up = 1,
    Down = 2,
    Left = 4,
    Right = 8,
}

#[derive(Debug, Clone, Copy)]
struct Command {
    dir: Dir,
    len: i64,
}

impl FromStr for Command {
    type Err = Error;
    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let parts = s.split_ascii_whitespace().collect::<Vec<_>>();
        if parts.len() != 3 {
            return Err(Error::FormatError);
        }
        let dir = match parts[0] {
            "R" => Dir::Right,
            "L" => Dir::Left,
            "D" => Dir::Down,
            "U" => Dir::Up,
            _ => { return Err(Error::FormatError); }
        };
        let len = parts[1].parse()?;
        Ok(Self { dir, len })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point(i64, i64);

impl Mul<i64> for Point {
    type Output = Point;
    fn mul(self, rhs: i64) -> Self::Output {
        Point(self.0 * rhs, self.1 * rhs)
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self( self.0 + rhs.0, self.1 + rhs.1 )
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

fn square(a: &Point, b: &Point) -> i64 {
    a.0 * b.1 - a.1 * b.0
}

fn perimeter(a: &Point, b: &Point) -> i64 {
   (b.1 - a.1 + b.0 - a.0).abs()
}

fn oriented_point(dir: Dir) -> Point {
    match dir {
        Dir::Down => Point(0, -1),
        Dir::Up => Point(0, 1),
        Dir::Left => Point(-1, 0),
        Dir::Right => Point(1, 0),
    }
}

fn parse_hex_command(s: &str) -> Result<Command> {
    let cmd = s.split_ascii_whitespace().last().unwrap();
    let len = i64::from_str_radix(&cmd[2..7], 16)?;
    let dir = match &cmd[7..8] {
        "0" => Ok(Dir::Right),
        "1" => Ok(Dir::Down),
        "2" => Ok(Dir::Left),
        "3" => Ok(Dir::Up),
        _ => Err(Error::FormatError)
    }?;
    Ok(Command {dir, len })
}

fn outter_area(commands: &[Command]) -> i64 {
    let mut p = Point(0, 0);
    let points = std::iter::once(Point(0, 0)).chain(commands.iter().map(|Command{dir, len}| {
        p += oriented_point(*dir) * *len;
        p
    })).collect::<Vec<_>>();
    assert_eq!(points.first(), points.last());

    let area = points.windows(2).map(|w| square(&w[0], &w[1])).sum::<i64>().abs();
    let perim = points.windows(2).map(|w| perimeter(&w[0], &w[1])).sum::<i64>();
    area / 2 + perim / 2 + 1
}

pub fn task1<S: AsRef<str>>(lines: &[S]) -> Result<i64> {
    let commands = lines.iter().map(|s| s.as_ref().parse()).collect::<Result<Vec<Command>>>()?;
    Ok(outter_area(&commands))
}

pub fn task2<S: AsRef<str>>(lines: &[S]) -> Result<i64> {
    let commands = lines.iter().map(|s| parse_hex_command(s.as_ref())).collect::<Result<Vec<Command>>>()?;
    Ok(outter_area(&commands))
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str =
r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#;

    #[test]
    fn test_task1() {
        let lines = DATA.lines().collect::<Vec<_>>();
        assert_eq!(Ok(62), task1(&lines));
    }

    #[test]
    fn test_task2() {
        let lines = DATA.lines().collect::<Vec<_>>();
        assert_eq!(Ok(952408144115), task2(&lines));
    }
}
