use std::collections::HashMap;
use std::collections::HashSet;

extern crate indoc;
use indoc::indoc;

mod utils;

fn main() {
    let contents = utils::load_input("inputs/day7.txt");
    let result = resolve(&contents, 5, 60);

    println!("{}", result);
}

fn extract_step_and_dep(line: &str) -> (char, char) {
    let chars: Vec<char> = line.chars().collect();
    (chars[5], chars[36])
}

fn dep_graph(input: &str) -> HashMap<char, HashSet<char>> {
    let mut result = HashMap::new();
    for line in input.lines() {
        let (requirement, required_by) = extract_step_and_dep(line);

        result.entry(requirement).or_insert(HashSet::new());

        let mut dependencies = result.entry(required_by).or_insert(HashSet::new());
        dependencies.insert(requirement);
    }
    result
}

fn next_up(dep_graph: &HashMap<char, HashSet<char>>, resolved: &HashSet<char>) -> Vec<char> {
    let mut possible = Vec::new();
    for (step, deps) in dep_graph {
        if deps.is_subset(resolved) {
            possible.push(*step);
        }
    }
    possible.sort();
    possible.reverse();
    possible
}


fn resolve(input: &str, workers: usize, base_cost: usize) -> isize {
    let mut dep_graph = dep_graph(input);
    let mut resolved = HashSet::new();
    let mut resolving = HashMap::new();

    let mut time = -1;
    while !dep_graph.is_empty() {
        time += 1;
        for time_left in resolving.values_mut() {
            *time_left -= 1;
        }
        resolving.retain(|&dep, time| {
            if *time == 0 {
                resolved.insert(dep);
                return false;
            } else {
                return true;
            }
        });
        let mut nexts = next_up(&dep_graph, &resolved);
        while resolving.len() < workers && nexts.len() > 0 {
            let next = nexts.pop().unwrap();
            dep_graph.remove(&next);
            resolving.insert(next, cost(next, base_cost));
        }
    }
    while !resolving.is_empty() {
        time += 1;
        for time_left in resolving.values_mut() {
            *time_left -= 1;
        }
        resolving.retain(|&dep, time| {
            if *time == 0 {
                resolved.insert(dep);
                return false;
            } else {
                return true;
            }
        });
    }

    time
}

fn cost(val: char, base: usize) -> usize {
    (val as usize) - 64 + base
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_line() {
        assert_eq!(('C', 'A'), extract_step_and_dep("Step C must be finished before step A can begin."));
        assert_eq!(('F', 'E'), extract_step_and_dep("Step F must be finished before step E can begin."));
    }

    #[test]
    fn extracts_all_deps() {
        let mut expected = HashMap::new();
        expected.insert('C', HashSet::new());
        expected.insert('A', ['C'].iter().cloned().collect());
        expected.insert('B', ['A'].iter().cloned().collect());
        expected.insert('D', ['A'].iter().cloned().collect());
        expected.insert('F', ['C'].iter().cloned().collect());
        expected.insert('E', ['B', 'D', 'F'].iter().cloned().collect());

        let result = dep_graph(SAMPLE);
        assert_eq!(expected, result);
    }

    #[test]
    fn next_resolved() {
        let mut dep_graph = dep_graph(SAMPLE);
        let mut resolved = HashSet::new();
        assert_eq!(vec!('C'), next_up(&dep_graph, &resolved));

        resolved.insert('C');
        dep_graph.remove(&'C');
        assert_eq!(vec!('F', 'A'), next_up(&dep_graph, &resolved));

    }

    #[test]
    fn calculates_cost() {
        assert_eq!(1, cost('A', 0));
        assert_eq!(86, cost('Z', 60));
    }

    #[test]
    fn resolves() {
        assert_eq!(15, resolve(SAMPLE, 2, 0));
    }

    static SAMPLE: &str = indoc!("
        Step C must be finished before step A can begin.
        Step C must be finished before step F can begin.
        Step A must be finished before step B can begin.
        Step A must be finished before step D can begin.
        Step B must be finished before step E can begin.
        Step D must be finished before step E can begin.
        Step F must be finished before step E can begin.
    ");

}