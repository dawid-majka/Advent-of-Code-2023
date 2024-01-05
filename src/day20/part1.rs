use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
enum Type {
    FlipFlop,
    Conjunction,
    Broadcaster,
}

#[derive(Debug, Clone)]
struct Module {
    name: String,
    module_type: Type,
    on: bool,
    state: HashMap<String, Pulse>,
    output: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
enum Pulse {
    High,
    Low,
}

impl Module {
    fn update(&mut self, name: &str, pulse: &Pulse) {
        match &self.module_type {
            Type::FlipFlop => {
                if *pulse == Pulse::Low {
                    self.on = !self.on;
                }
            }
            Type::Conjunction => {
                self.state.insert(name.to_string(), pulse.clone());
            }
            Type::Broadcaster => {}
        }
    }

    fn emit(&self, q: &mut VecDeque<(String, String, Pulse)>) {
        let pulse = match self.module_type {
            Type::FlipFlop => match self.on {
                true => Pulse::High,
                false => Pulse::Low,
            },
            Type::Conjunction => {
                if self.state.values().all(|x| *x == Pulse::High) {
                    Pulse::Low
                } else {
                    Pulse::High
                }
            }
            Type::Broadcaster => Pulse::Low,
        };

        for output in self.output.iter() {
            q.push_back((self.name.clone(), output.to_string(), pulse.clone()));
        }
    }
}

pub fn solve(input: &str) -> usize {
    let destinations_map: HashMap<&str, Vec<&str>> = input
        .lines()
        .map(|line| {
            let (source, destinations) = line
                .split_once(" -> ")
                .expect("Input should have valid structure.");

            let destinations = destinations.split(", ").collect();

            if source == "broadcaster" {
                (source, destinations)
            } else {
                let (_, name) = source.split_at(1);
                (name, destinations)
            }
        })
        .collect();

    let mut modules: HashMap<String, Module> = input
        .lines()
        .map(|line| {
            let (source, destinations) = line
                .split_once(" -> ")
                .expect("Input should have valid structure.");

            let destinations: Vec<String> = destinations
                .split(',')
                .map(|dst| dst.trim().to_string())
                .collect();

            if source == "broadcaster" {
                (
                    "broadcaster".to_string(),
                    Module {
                        name: "broadcaster".to_string(),
                        module_type: Type::Broadcaster,
                        on: true,
                        state: HashMap::new(),
                        output: destinations,
                    },
                )
            } else {
                let (t, name) = source.split_at(1);

                let module = match t {
                    "%" => Module {
                        name: name.to_string(),
                        module_type: Type::FlipFlop,
                        on: false,
                        state: HashMap::new(),
                        output: destinations,
                    },
                    "&" => Module {
                        name: name.to_string(),
                        module_type: Type::Conjunction,
                        on: true,
                        state: HashMap::new(),
                        output: destinations,
                    },
                    _ => panic!("Module type do not exists."),
                };

                (name.to_string(), module)
            }
        })
        .collect();

    let broadcaster = modules
        .get("broadcaster")
        .expect("Broadcaster should exist.")
        .clone();

    for (src, destinations) in destinations_map.iter() {
        for dst in destinations {
            if let Some(m) = modules.get_mut(&dst.to_string()) {
                if m.module_type == Type::Conjunction {
                    m.state.insert(src.to_string(), Pulse::Low);
                }
            }
        }
    }

    let mut high_count = 0;
    let mut low_count = 0;

    for _ in 0..1000 {
        low_count += 1;
        let mut q = VecDeque::new();

        for b_out in broadcaster.output.iter() {
            q.push_back(("broadcaster".to_string(), b_out.clone(), Pulse::Low));
        }

        while let Some((src, dst, pulse)) = q.pop_front() {
            match pulse {
                Pulse::Low => low_count += 1,
                Pulse::High => high_count += 1,
            }

            if let Some(module) = modules.get_mut(&dst) {
                match module.module_type {
                    Type::FlipFlop => {
                        if pulse == Pulse::Low {
                            module.update(&src, &pulse);
                            module.emit(&mut q);
                        }
                    }
                    Type::Conjunction => {
                        module.update(&src, &pulse);
                        module.emit(&mut q);
                    }
                    Type::Broadcaster => {}
                }
            }
        }
    }

    low_count * high_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_riddle() {
        let input = include_str!("test_input1.txt");
        assert_eq!(solve(input), 32000000);
    }
    #[test]
    fn should_solve_riddle2() {
        let input = include_str!("test_input2.txt");
        assert_eq!(solve(input), 11687500);
    }
}
