use priority_queue::PriorityQueue;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn solve(input: &str) -> usize {
    let mut nodes: HashSet<&str> = HashSet::new();
    let mut edges: HashMap<&str, HashMap<&str, usize>> = HashMap::new();

    input.lines().for_each(|line| {
        let (l_node, l_edges) = line
            .split_once(": ")
            .expect("Input lines to have valid format.");

        l_edges.split_ascii_whitespace().for_each(|edge| {
            nodes.insert(l_node);
            nodes.insert(edge);

            edges.entry(l_node).or_default().insert(edge, 1);
            edges.entry(edge).or_default().insert(l_node, 1);
        })
    });

    let mut c_nodes = nodes.clone();
    let mut c_edges = edges.clone();

    let mut best_phase = 0;
    let mut best_cut_weight = usize::MAX;
    let mut contractions = Vec::new();

    for phase in 0..nodes.len() - 1 {
        let mut cut_weight = 0;
        let mut s = "";
        let mut t = "";

        let mut queue = PriorityQueue::new();

        for &node in c_nodes.iter() {
            queue.push(node, 0);
        }

        while let Some((node, weight)) = queue.pop() {
            s = t;
            t = node;
            cut_weight = weight;

            for (edge, weight) in c_edges.get(node).unwrap() {
                queue.change_priority_by(edge, |cur| *cur += weight);
            }
        }

        if cut_weight < best_cut_weight {
            best_cut_weight = cut_weight;
            best_phase = phase;
        }

        contractions.push((s, t));

        let t_edges = c_edges.get(t).unwrap().clone();

        for (&node, &weight) in t_edges.iter() {
            *c_edges.entry(s).or_default().entry(node).or_insert(0) += weight;
            *c_edges.entry(node).or_default().entry(s).or_insert(0) += weight;
        }

        c_nodes.remove(t);
        c_edges.remove(t);

        c_edges.iter_mut().for_each(|(_, edges)| {
            edges.remove(t);
        });
    }

    let mut graph = HashMap::new();

    for (s, t) in contractions.iter().take(best_phase) {
        graph.entry(*s).or_insert_with(Vec::new).push(*t);
        graph.entry(*t).or_insert_with(Vec::new).push(*s);
    }

    let mut visited = HashSet::new();
    let mut q = VecDeque::new();

    q.push_back(contractions[best_phase].1);

    while let Some(node) = q.pop_front() {
        if visited.contains(node) {
            continue;
        }
        visited.insert(node);
        if let Some(edges) = graph.get(node) {
            for edge in edges {
                q.push_back(*edge);
            }
        }
    }

    let partition: Vec<&str> = visited.into_iter().collect();

    partition.len() * (nodes.len() - partition.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_riddle() {
        let input = include_str!("test_input1.txt");
        assert_eq!(solve(input), 54);
    }
}
