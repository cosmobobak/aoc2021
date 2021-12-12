use std::collections::HashSet;

use crate::util::get_task;

const START_ID: &str = "start";
const END_ID: &str = "end";

struct Node {
    id: String,
    children: Vec<usize>,
}

impl Node {
    fn is_big(&self) -> bool {
        self.id.chars().all(char::is_uppercase)
    }
}

fn count_paths(nodes: &[Node], from: usize, to: usize, visited: &mut HashSet<usize>) -> usize {
    if from == to { return 1; }
    let mut count = 0;

    if !nodes[from].is_big() { visited.insert(from); }
    for child in nodes[from].children.iter() {
        if visited.contains(child) {
            continue;
        }
        count += count_paths(nodes, *child, to, visited);
    }
    if !nodes[from].is_big() { visited.remove(&from); }

    count
}

fn count_paths_with_double(nodes: &[Node], from: usize, to: usize, visited: &mut HashSet<usize>, double_used: Option<usize>) -> usize {
    // same as count_paths, except we can use a small node twice
    if from == to {
        if let Some(used) = double_used {
            return visited.contains(&used) as usize;
        }
        return 1;
    }
    let mut count = 0;

    // first count the paths if we don't use the node twice
    if !nodes[from].is_big() { visited.insert(from); }
    for child in nodes[from].children.iter() {
        if visited.contains(child) {
            continue;
        }
        count += count_paths_with_double(nodes, *child, to, visited, double_used);
    }
    if !nodes[from].is_big() { visited.remove(&from); }

    // then count the paths if we do use the node twice
    if double_used.is_none() && !nodes[from].is_big() && nodes[from].id != START_ID && nodes[from].id != END_ID {
        for child in nodes[from].children.iter() {
            if visited.contains(child) {
                continue;
            }
            count += count_paths_with_double(nodes, *child, to, visited, Some(from));
        }
    }

    count
}

pub fn task12() {
    let start = std::time::Instant::now();
    // io
    let input = get_task(12);
    let edges: Vec<(String, String)> = input
        .lines()
        .map(|line| {
            let mut parts = line.split('-');
            let from = parts.next().unwrap().to_string();
            let to = parts.next().unwrap().to_string();
            (from, to)
        })
        .collect();

    // setup for both
    
    let node_ids = edges.clone().into_iter().flat_map(|(from, to)| vec![from, to]).collect::<HashSet<_>>();
    let mut graph = node_ids.into_iter().map(|id| Node { id, children: vec![] }).collect::<Vec<_>>();
    for (from, to) in edges.iter() {
        let from_node = graph.iter().position(|node| node.id == *from).unwrap();
        let to_node = graph.iter().position(|node| node.id == *to).unwrap();
        graph[from_node].children.push(to_node);
        graph[to_node].children.push(from_node);
    }
    let graph = graph;
    let start_pos = graph.iter().position(|node| node.id == *START_ID).unwrap();
    let end_pos = graph.iter().position(|node| node.id == *END_ID).unwrap();

    // task 1

    let count = count_paths(&graph, start_pos, end_pos, &mut HashSet::new());
    
    println!("Task 1: {}", count);

    // task 2
    
    let count_with_doubling = count_paths_with_double(&graph, start_pos, end_pos, &mut HashSet::new(), None);
    println!("Task 2: {}", count_with_doubling);

    println!("done in {}us!", start.elapsed().as_micros());
}
