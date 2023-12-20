use std::{str::FromStr, collections::{HashMap, VecDeque}};

#[derive(thiserror::Error, Debug, Clone, Eq, PartialEq)]
pub enum Error {
    #[error("Format error")]
    FormatError,
    #[error("Parse int error")]
    ParseIntError(#[from] std::num::ParseIntError),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
enum RelayType {
    Broadcaster,
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
    Dummy,
}

impl RelayType {
    fn process(&mut self, input_name: &str, input_signal: bool) -> Option<bool> {
        match self {
            RelayType::Broadcaster => Some(false),
            RelayType::FlipFlop(state) => {
                if input_signal { return None; }
                *state = !*state;
                Some(*state)
            },
            RelayType::Conjunction(input) => {
                *input.get_mut(input_name).unwrap() = input_signal;
                Some(!input.values().all(|v| *v))
            },
            RelayType::Dummy => {
                None
            }
        }
    }
}

#[derive(Debug)]
struct Module {
    name: String,
    relay_type: RelayType,
    output: Vec<String>,
}

impl FromStr for Module {
    type Err = Error;
    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let (module, targets) = s.split_once(" -> ").ok_or(Error::FormatError)?;
        let (relay_type, name) =
            if module.starts_with('%') { (RelayType::FlipFlop(false), &module[1..]) }
            else if module.starts_with('&') { (RelayType::Conjunction(HashMap::new()), &module[1..])}
            else if module == "broadcaster" { (RelayType::Broadcaster, module) }
            else { (RelayType::Dummy, module)};
        Ok(Module { name: name.to_string(), relay_type, output: targets.split(", ").map(|s| s.to_string()).collect()})
    }
}

fn fill_inputs(relays: &mut HashMap<String, Module>) {
    let mut inputs: HashMap<String, Vec<String>> = HashMap::new();
    for module in relays.values() {
        for target in &module.output {
            inputs.entry(target.clone()).or_default().push(module.name.clone());
        }
    }
    for module in relays.values_mut() {
        if let RelayType::Conjunction(conj) = &mut module.relay_type {
            *conj = inputs.get(&module.name).unwrap().iter().map(|name| (name.clone(), false)).collect();
        }
    }
}

fn run<F: FnMut(&str, &str, bool)>(relays: &mut HashMap<String, Module>, mut f: F) {
    let mut q = VecDeque::new();

    q.push_back(("broadcaster".to_owned(), String::from("button"), false));
    while let Some((module_name, input_name, input_signal)) = q.pop_front() {
        if let Some(module) = relays.get_mut(&module_name) {
            if let Some(output_signal) = module.relay_type.process(&input_name, input_signal) {
                for target in &module.output {
                    f(&module_name, target, output_signal);
                    q.push_back((target.clone(), module_name.clone(), output_signal))
                }
            }
        }
    }
}

pub fn task1<S: AsRef<str>>(lines: &[S]) -> Result<u64> {
    let mut relays: HashMap<String, Module> = lines.iter().map(|s| s.as_ref().parse().unwrap()).map(|m: Module| (m.name.clone(), m)).collect();
    fill_inputs(&mut relays);
    let mut low_count = 0;
    let mut high_count = 0;
    for _ in 0..1000 {
        low_count += 1;
        run(&mut relays, |_, _, signal| {
            if signal {
                high_count += 1;
            } else {
                low_count += 1;
            }
        });
    }
    Ok(low_count * high_count)
}

pub fn task2<S: AsRef<str>>(lines: &[S]) -> Result<u64> {
    let mut relays: HashMap<String, Module> = lines.iter().map(|s| s.as_ref().parse().unwrap()).map(|m: Module| (m.name.clone(), m)).collect();
    fill_inputs(&mut relays);
    let mut count = 0;
    let mut seen = HashMap::new();
    let mut stop = false;
    loop {
        count += 1;
        run(&mut relays, |output_name, input_name, signal| {
            if input_name == "sq" && signal {
                seen.insert(output_name.to_string(), count);
                if ["kk", "xr", "fv", "vt"].into_iter().all(|k| seen.contains_key(k)) {
                    stop = true;
                }
            }
        });
        if stop { break; }
    }
    Ok(seen.values().product())
    // 217317393039529
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str =
r#"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"#;

    #[test]
    fn test_task1() {
        let lines = DATA.lines().collect::<Vec<_>>();
        assert_eq!(Ok(11687500), task1(&lines));
    }
}
