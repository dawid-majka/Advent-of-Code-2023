use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Rule {
    category: String,
    sign: String,
    value: usize,
    destination: String,
}

pub fn solve(input: &str) -> usize {
    let chunks: Vec<&str> = input.split("\n\n").collect();

    let mut workflows: HashMap<&str, Vec<Rule>> = chunks[0]
        .lines()
        .map(|line| {
            let (name, rules) = line.split_once('{').expect("Rulest to be valid.");

            let rules: Vec<Rule> = rules[..rules.len() - 1]
                .split(',')
                .map(|rule| {
                    if rule.contains(':') {
                        let sign_index = rule
                            .find('<')
                            .or_else(|| rule.find('>'))
                            .expect("Rule to have sign index.");

                        let (category, rest) = rule.split_at(sign_index);

                        let sign = &rest[..1];
                        let (value, destination) = rest[1..]
                            .split_once(':')
                            .expect("Rule to have correct elements.");

                        let value = value.parse::<usize>().expect("Value to be valid number.");

                        Rule {
                            category: category.to_string(),
                            sign: sign.to_string(),
                            value,
                            destination: destination.to_string(),
                        }
                    } else {
                        Rule {
                            category: "0".to_string(),
                            sign: "0".to_string(),
                            value: 0,
                            destination: rule.to_string(),
                        }
                    }
                })
                .collect();

            (name, rules)
        })
        .collect();

    let w_name = "in";

    let steps: Vec<Rule> = Vec::new();

    let all_paths = next_rule(w_name, &steps, &mut workflows);

    calculate_combinations(&all_paths)
}

fn calculate_combinations(paths: &[Vec<Rule>]) -> usize {
    let mut sum = 0;

    for path in paths {
        let mut x_min = 1;
        let mut m_min = 1;
        let mut a_min = 1;
        let mut s_min = 1;

        let mut x_max = 4000;
        let mut m_max = 4000;
        let mut a_max = 4000;
        let mut s_max = 4000;

        for rule in path {
            match rule.category.as_str() {
                "x" => match rule.sign.as_str() {
                    "<" => {
                        if rule.value - 1 < x_max {
                            x_max = rule.value - 1;
                        }
                    }
                    "<=" => {
                        if rule.value < x_max {
                            x_max = rule.value;
                        }
                    }
                    ">" => {
                        if rule.value + 1 > x_min {
                            x_min = rule.value + 1;
                        }
                    }
                    ">=" => {
                        if rule.value > x_min {
                            x_min = rule.value;
                        }
                    }
                    _ => {}
                },
                "m" => match rule.sign.as_str() {
                    "<" => {
                        if rule.value - 1 < m_max {
                            m_max = rule.value - 1;
                        }
                    }
                    "<=" => {
                        if rule.value < m_max {
                            m_max = rule.value;
                        }
                    }
                    ">" => {
                        if rule.value + 1 > m_min {
                            m_min = rule.value + 1;
                        }
                    }
                    ">=" => {
                        if rule.value > m_min {
                            m_min = rule.value;
                        }
                    }
                    _ => {}
                },
                "a" => match rule.sign.as_str() {
                    "<" => {
                        if rule.value - 1 < a_max {
                            a_max = rule.value - 1;
                        }
                    }
                    "<=" => {
                        if rule.value < a_max {
                            a_max = rule.value;
                        }
                    }
                    ">" => {
                        if rule.value + 1 > a_min {
                            a_min = rule.value + 1;
                        }
                    }
                    ">=" => {
                        if rule.value > a_min {
                            a_min = rule.value;
                        }
                    }
                    _ => {}
                },
                "s" => match rule.sign.as_str() {
                    "<" => {
                        if rule.value - 1 < s_max {
                            s_max = rule.value - 1;
                        }
                    }
                    "<=" => {
                        if rule.value < s_max {
                            s_max = rule.value;
                        }
                    }
                    ">" => {
                        if rule.value + 1 > s_min {
                            s_min = rule.value + 1;
                        }
                    }
                    ">=" => {
                        if rule.value > s_min {
                            s_min = rule.value;
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        sum +=
            (x_max - x_min + 1) * (m_max - m_min + 1) * (a_max - a_min + 1) * (s_max - s_min + 1);
    }

    sum
}

fn next_rule(
    w_name: &str,
    steps: &[Rule],
    workflows: &mut HashMap<&str, Vec<Rule>>,
) -> Vec<Vec<Rule>> {
    let rules = workflows.get(w_name).expect("Workflow to exist").to_owned();

    let mut current_steps: Vec<Rule> = Vec::new();

    let mut all_paths: Vec<Vec<Rule>> = Vec::new();

    current_steps.extend(steps.to_vec());

    for rule in rules.iter() {
        if rule.category == "0" {
            if rule.destination == "A" {
                all_paths.push(current_steps.clone());
            } else if rule.destination == "R" {
            } else {
                all_paths.extend(next_rule(&rule.destination, &current_steps, workflows));
            }
        } else {
            let opposite = match rule.sign.as_str() {
                ">" => "<=",
                "<" => ">=",
                _ => "",
            };

            let opposite_rule = Rule {
                category: rule.category.clone(),
                sign: opposite.to_string(),
                value: rule.value,
                destination: "next rule name".to_string(),
            };

            let mut c1 = current_steps.clone();
            c1.push(rule.clone());
            current_steps.push(opposite_rule.clone());

            if rule.destination == "A" {
                all_paths.push(c1.clone());
            } else if rule.destination == "R" {
            } else {
                all_paths.extend(next_rule(&rule.destination, &c1, workflows));
            }
        }
    }

    all_paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_riddle() {
        let input = include_str!("test_input1.txt");
        assert_eq!(solve(input), 167409079868000);
    }
}
