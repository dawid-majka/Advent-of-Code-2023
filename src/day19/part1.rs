use std::collections::HashMap;

// #[derive(Debug)]
// struct Part {
//     x: usize,
//     m: usize,
//     a: usize,
//     s: usize,
// }

#[derive(Debug)]
struct Rule<'a> {
    category: &'a str,
    sign: &'a str,
    value: usize,
    destination: &'a str,
}

pub fn solve(input: &str) -> usize {
    let chunks: Vec<&str> = input.split("\n\n").collect();

    let workflows: HashMap<&str, Vec<Rule>> = chunks[0]
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
                            category,
                            sign,
                            value,
                            destination,
                        }
                    } else {
                        Rule {
                            category: "0",
                            sign: "0",
                            value: 0,
                            destination: rule,
                        }
                    }
                })
                .collect();

            (name, rules)
        })
        .collect();

    let parts: Vec<HashMap<&str, usize>> = chunks[1]
        .lines()
        .map(|line| {
            let line = &line[1..line.len() - 1];
            let mut map = HashMap::new();

            line.split(',').for_each(|category| {
                let rating = category.split_once('=').expect("Part ratings to be valid.");

                map.insert(
                    rating.0,
                    rating
                        .1
                        .parse::<usize>()
                        .expect("Rating to be valid number."),
                );
            });

            map
        })
        .collect();

    let accepted: Vec<HashMap<&str, usize>> = parts
        .into_iter()
        .filter(|part| {
            let mut w_name = "in";

            loop {
                let workflow = workflows.get(w_name).expect("Workflow to exist");

                for rule in workflow {
                    if rule.category == "0" {
                        if rule.destination == "A" {
                            return true;
                        } else if rule.destination == "R" {
                            return false;
                        } else {
                            w_name = rule.destination;
                        }
                    } else {
                        let category_value = part
                            .get(rule.category)
                            .expect("Parts category to match rule category");

                        match rule.sign {
                            ">" => {
                                if category_value > &rule.value {
                                    if rule.destination == "A" {
                                        return true;
                                    }
                                    if rule.destination == "R" {
                                        return false;
                                    }
                                    w_name = rule.destination;
                                    break;
                                }
                            }
                            "<" => {
                                if category_value < &rule.value {
                                    if rule.destination == "A" {
                                        return true;
                                    }
                                    if rule.destination == "R" {
                                        return false;
                                    }
                                    w_name = rule.destination;
                                    break;
                                }
                            }
                            _ => panic!("Invalid sign in rule"),
                        }
                    }
                }
            }
        })
        .collect();

    let sum: usize = accepted.iter().map(|map| map.values().sum::<usize>()).sum();

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_riddle() {
        let input = include_str!("test_input1.txt");
        assert_eq!(solve(input), 19114);
    }
}
