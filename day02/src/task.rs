#[derive(thiserror::Error, Debug, Clone, Copy, Eq, PartialEq)]
pub enum Error {
    #[error("Parse error")]
    ParseError,
}

pub type Result<T> = std::result::Result<T, Error>;

fn is_valid_game(cubes: &[u32; 3]) -> bool {
    let max_possible = &[12, 13, 14];
    cubes.iter().zip(max_possible.iter()).all(|(a, b)| a <= b)
}

fn color_number(color: &str) -> Result<usize> {
    match color {
        "red" => Ok(0),
        "green" => Ok(1),
        "blue" => Ok(2),
        _ => Err(Error::ParseError)
    }
}

fn power(cubes: &[u32; 3]) -> u32 {
    cubes.iter().product()
}

fn parse_line(line: &str) -> Result<(u32, Vec<[u32; 3]>)> {
    let (game_number, sets) = line.split_once(':').ok_or(Error::ParseError)?;
    let n = game_number.trim().split_ascii_whitespace().last().and_then(|s| s.parse::<u32>().ok()).ok_or(Error::ParseError)?;
    let mut game = Vec::new();
    for set in sets.split(';') {
        let cubes: Result<[u32; 3]> = set.split(',').try_fold([0u32; 3], |mut acc, s| {
            let (counts, color) = s.trim().split_once(' ').ok_or(Error::ParseError)?;
            let count = counts.parse::<u32>().map_err(|_| Error::ParseError)?;
            acc[color_number(color)?] = count;
            Ok(acc)
        });
        game.push(cubes?);
    }
    Ok((n, game))
}

pub fn task1<S: AsRef<str>>(lines: &[S]) -> Result<u32> {
    let mut acc = 0;
    for line in lines.iter() {
        let (n, game) = parse_line(line.as_ref())?;
        if game.iter().all(|g| is_valid_game(g)) {
            acc += n;
        }
    }
    Ok(acc)
}

pub fn task2<S: AsRef<str>>(lines: &[S]) -> Result<u32> {
    lines.iter()
    .map(|s| parse_line(s.as_ref()))
    .try_fold(0, |acc, res| {
        match res {
            Ok((_, game)) => {
                let score = game.iter().fold([0u32; 3], |mut acc, g| {
                    g.iter().enumerate().for_each(|(i, x)| acc[i] = acc[i].max(*x));
                    acc
                });
                Ok(acc + power(&score))
            },
            Err(err) => Err(err)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_example() {
        let lines = DATA.lines().map(|s| s.trim()).collect::<Vec<_>>();
        assert_eq!(Ok(8), task1(&lines));
    }

    #[test]
    fn test_task2() {
        let lines = DATA.lines().map(|s| s.trim()).collect::<Vec<_>>();
        assert_eq!(Ok(2286), task2(&lines));
    }

    #[test]
    fn test_validate() {
        assert!(is_valid_game(&[12, 13, 14]));
        assert!(!is_valid_game(&[2, 3, 16]));
    }
}
