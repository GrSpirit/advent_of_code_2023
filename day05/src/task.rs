use std::num::ParseIntError;

#[derive(thiserror::Error, Debug, Clone, Eq, PartialEq)]
pub enum Error {
    #[error("Parse int error")]
    ParseIntError(#[from] ParseIntError),
    #[error("Format error")]
    FormatError,
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn task1<S: AsRef<str>>(lines: &[S]) -> Result<u64> {
    let seeds = lines[0].as_ref().split_ascii_whitespace().skip(1).map(|n| n.parse::<u64>().unwrap()).collect::<Vec<_>>();
    let mut pos = 1;
    let mut parse_map = || {
        pos += 2;
        let mut map = Vec::new();
        while pos < lines.len() && !lines[pos].as_ref().is_empty() {
            let nums = lines[pos].as_ref().split_ascii_whitespace().map(|n| n.parse::<u64>().unwrap()).collect::<Vec<_>>();
            map.push(((nums[1]..nums[1]+nums[2]), (nums[0]..nums[0]+nums[2])));
            pos += 1;
        }
        map.sort_by_key(|(r, _)| r.start);
        map
    };
    let maps = (0..7).map(|_| parse_map()).collect::<Vec<_>>();
    let mut ans = u64::MAX;
    for mut seed in seeds {
        for map in &maps {
            let m = map.partition_point(|(range_from, _)| range_from.end <= seed);
            if let Some((range_from, range_to)) = map.get(m) {
                if range_from.contains(&seed) {
                    seed = seed + range_to.start - range_from.start;
                }
            }
        }
        ans = ans.min(seed);
    }
    Ok(ans)
}

pub fn task2<S: AsRef<str>>(lines: &[S]) -> Result<u64> {
    // let seeds = lines[0].as_ref().split_ascii_whitespace().skip(1).map(|n| n.parse::<u64>().unwrap()).collect::<Vec<_>>();
    let seeds = lines[0].as_ref().split_ascii_whitespace().skip(1).map(|n| n.parse::<u64>().unwrap()).collect::<Vec<_>>();
    let seed_ranges = seeds.chunks(2).map(|chunk| (chunk[0]..chunk[0]+chunk[1])).collect::<Vec<_>>();
    let mut pos = 1;
    let mut parse_map = || {
        pos += 2;
        let mut map = Vec::new();
        while pos < lines.len() && !lines[pos].as_ref().is_empty() {
            let nums = lines[pos].as_ref().split_ascii_whitespace().map(|n| n.parse::<u64>().unwrap()).collect::<Vec<_>>();
            map.push(((nums[1]..nums[1]+nums[2]), (nums[0]..nums[0]+nums[2])));
            pos += 1;
        }
        map.sort_by_key(|(r, _)| r.start);
        map
    };
    let maps = (0..7).map(|_| parse_map()).collect::<Vec<_>>();
    let mut ans = u64::MAX;
    for seed_range in seed_ranges {
        for mut seed in seed_range {
            for map in &maps {
                let m = map.partition_point(|(range_from, _)| range_from.end <= seed);
                if let Some((range_from, range_to)) = map.get(m) {
                    if range_from.contains(&seed) {
                        seed = seed + range_to.start - range_from.start;
                    }
                }
            }
            ans = ans.min(seed);
        }
    }
    Ok(ans)
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str =
"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_task1() {
        let lines = DATA.lines().collect::<Vec<_>>();
        assert_eq!(Ok(35), task1(&lines));
    }

    #[test]
    fn test_task2() {
        let lines = DATA.lines().collect::<Vec<_>>();
        assert_eq!(Ok(46), task2(&lines));
    }

}
