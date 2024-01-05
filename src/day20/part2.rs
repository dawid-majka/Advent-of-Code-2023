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

    let emits_to_rx: Vec<Module> = modules
        .values()
        .cloned()
        .filter(|m| m.output.contains(&"rx".to_string()))
        .collect();

    assert!(
        1 == emits_to_rx.len(),
        "rx should get pulses only from one module"
    );

    let mut cycles: HashMap<String, usize> = HashMap::new();
    let mut seen: HashMap<String, usize> = HashMap::new();

    for (name, dst) in destinations_map {
        if dst.contains(&emits_to_rx[0].name.as_str()) {
            seen.insert(name.to_string(), 0);
        }
    }

    let mut press_count = 0;

    'outer: loop {
        let mut q = VecDeque::new();

        press_count += 1;

        for b_out in broadcaster.output.iter() {
            q.push_back(("broadcaster".to_string(), b_out.clone(), Pulse::Low));
        }

        while let Some((src, dst, pulse)) = q.pop_front() {
            if let Some(module) = modules.get_mut(&dst) {
                if module.name == emits_to_rx[0].name && pulse == Pulse::High {
                    seen.entry(src.clone()).and_modify(|elem| *elem += 1);

                    if !cycles.contains_key(&src) {
                        cycles.insert(src.clone(), press_count);
                    } else {
                        assert_eq!(
                            press_count,
                            seen.get(&src).unwrap() * cycles.get(&src).unwrap()
                        );
                    }

                    if seen.values().all(|&elem| elem != 0) {
                        let values: Vec<usize> = cycles.values().cloned().collect();
                        let lcm = lcm(&values);

                        break 'outer lcm;
                    }
                }

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
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}
