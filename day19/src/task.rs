use std::{str::FromStr, collections::HashMap, ops::Range};

#[derive(thiserror::Error, Debug, Clone, Eq, PartialEq)]
pub enum Error {
    #[error("Format error")]
    FormatError,
    #[error("Parse int error")]
    ParseIntError(#[from] std::num::ParseIntError),
    // #[error("Unknown workflow")]
    // UnknownWorkflow,
}

pub type Result<T> = std::result::Result<T, Error>;

const MAX_RANGE: i32 = 4001;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Target {
    Workflow(String),
    Accept,
    Reject,
}

impl FromStr for Target {
    type Err = Error;
    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        match s {
            "A" => Ok(Target::Accept),
            "R" => Ok(Target::Reject),
            _ => Ok(Target::Workflow(s.to_string()))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Xmas { X, M, A, S }

impl FromStr for Xmas {
    type Err = Error;
    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        match s {
            "x" => Ok(Xmas::X),
            "m" => Ok(Xmas::M),
            "a" => Ok(Xmas::A),
            "s" => Ok(Xmas::S),
            _ => Err(Error::FormatError),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Command {
    Less(Xmas, i32, Target),
    Greater(Xmas, i32, Target),
    Default(Target),
}

impl FromStr for Command {
    type Err = Error;
    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        match s.split_once(':') {
            Some((condition, target)) => {
                if let Some((xmas_code, val)) = condition.split_once('<') {
                    Ok(Command::Less(xmas_code.parse()?, val.parse()?, target.parse()?))
                } else if let Some((xmas_code, val)) = condition.split_once('>') {
                    Ok(Command::Greater(xmas_code.parse()?, val.parse()?, target.parse()?))
                } else {
                    Err(Error::FormatError)
                }
            },
            None => Ok(Command::Default(s.parse()?))
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Part {
    x: i32, m: i32, a: i32, s: i32
}

impl Part {
    fn get(&self, xmas: &Xmas) -> i32 {
        match xmas {
            Xmas::X => self.x,
            Xmas::M => self.m,
            Xmas::A => self.a,
            Xmas::S => self.s,
        }
    }

    fn sum(&self) -> i32 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug, Clone)]
struct PartRanges {
    x: Range<i32>,
    m: Range<i32>,
    a: Range<i32>,
    s: Range<i32>,
}

fn intersect_ranges<T: PartialOrd + Ord>(lhs: Range<T>, rhs: Range<T>) -> Range<T> {
    let start = lhs.start.max(rhs.start);
    let end = lhs.end.min(rhs.end);
    Range { start, end }
}

impl PartRanges {
    fn product(&self) -> u64 {
        self.x.len() as u64 * self.m.len() as u64 * self.a.len() as u64 * self.s.len() as u64
    }

    fn is_empty(&self) -> bool {
        self.x.is_empty() || self.m.is_empty() || self.a.is_empty() || self.s.is_empty()
    }

    fn intersect(&self, xmas: Xmas, r: Range<i32>) -> PartRanges {
        let mut res= self.clone();
        match xmas {
            Xmas::X => res.x = intersect_ranges(res.x, r),
            Xmas::M => res.m = intersect_ranges(res.m, r),
            Xmas::A => res.a = intersect_ranges(res.a, r),
            Xmas::S => res.s = intersect_ranges(res.s, r),
        }
        res
    }
}


impl FromStr for Part {
    type Err = Error;
    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let ss = s.strip_prefix('{').ok_or(Error::FormatError)?.strip_suffix('}').ok_or(Error::FormatError)?;
        let parts = ss.split(',').collect::<Vec<_>>();
        let x = parts[0].strip_prefix("x=").ok_or(Error::FormatError)?.parse()?;
        let m = parts[1].strip_prefix("m=").ok_or(Error::FormatError)?.parse()?;
        let a = parts[2].strip_prefix("a=").ok_or(Error::FormatError)?.parse()?;
        let s = parts[3].strip_prefix("s=").ok_or(Error::FormatError)?.parse()?;
        Ok(Self { x, m, a, s })
    }
}

fn eval(commands: &HashMap<String, Vec<Command>>, target: &Target, part: Part) -> i32 {
    match target {
        Target::Accept => part.sum(),
        Target::Reject => 0,
        Target::Workflow(workflow) => {
            for cmd in commands.get(workflow).unwrap() {
                match cmd {
                    Command::Less(xmas, val, target) => {
                        if part.get(&xmas) < *val {
                            return eval(commands, target, part);
                        }
                    },
                    Command::Greater(xmas, val, target) => {
                        if part.get(&xmas) > *val {
                            return eval(commands, target, part);
                        }
                    },
                    Command::Default(target) => {
                        return eval(commands, target, part);
                    }
                }
            }
            return 0;
        }
    }
}

fn eval_range(commands: &HashMap<String, Vec<Command>>, target: &Target, mut part: PartRanges) -> u64 {
    match target {
        Target::Accept => part.product(),
        Target::Reject => 0,
        Target::Workflow(workflow) => {
            let mut acc = 0;
            for cmd in commands.get(workflow).unwrap() {
                if part.is_empty() { return acc; }
                match cmd {
                    Command::Less(xmas, val, next_target) => {
                        acc += eval_range(commands, next_target, part.intersect(*xmas, 1..*val));
                        part = part.intersect(*xmas, *val..MAX_RANGE);
                    },
                    Command::Greater(xmas, val, next_target) => {
                        acc += eval_range(commands, next_target, part.intersect(*xmas, *val + 1..MAX_RANGE));
                        part = part.intersect(*xmas, 1..*val + 1);
                    },
                    Command::Default(next_target) => {
                        acc += eval_range(commands, next_target, part.clone());
                    }
                }
            }
            return acc;
        }
    }
}

fn parse_commands<S: AsRef<str>>(lines: &[S]) -> HashMap<String, Vec<Command>> {
    lines.iter()
        .map(|s| {
            let pos = s.as_ref().chars().position(|c| c == '{').unwrap();
            let (name, cmnds) = s.as_ref().split_at(pos);
            let cmndv = cmnds
                .strip_prefix('{').unwrap()
                .strip_suffix('}').unwrap()
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect::<Vec<Command>>();
            (name.to_string(), cmndv)
        })
        .collect()
}

pub fn task1<S: AsRef<str>>(lines: &[S]) -> Result<i32> {
    let input = lines.split(|s| s.as_ref().is_empty()).collect::<Vec<_>>();
    let commands = parse_commands(&input[0]);
    let parts = input[1].iter().map(|s| s.as_ref().parse()).collect::<Result<Vec<Part>>>()?;
    return Ok(parts.into_iter().map(|part| eval(&commands, &Target::Workflow("in".to_string()), part)).sum());
}

pub fn task2<S: AsRef<str>>(lines: &[S]) -> Result<u64> {
    let input = lines.split(|s| s.as_ref().is_empty()).collect::<Vec<_>>();
    let commands = parse_commands(&input[0]);
    Ok(
        eval_range(
            &commands,
            &Target::Workflow("in".to_string()),
            PartRanges { x: 1..MAX_RANGE, m: 1..MAX_RANGE, a: 1..MAX_RANGE, s: 1..MAX_RANGE}
        )
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str =
r#"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"#;

    #[test]
    fn test_task1() {
        let lines = DATA.lines().collect::<Vec<_>>();
        assert_eq!(Ok(19114), task1(&lines));
    }

    #[test]
    fn test_task2() {
        let lines = DATA.lines().collect::<Vec<_>>();
        assert_eq!(Ok(167409079868000), task2(&lines));
    }
}
