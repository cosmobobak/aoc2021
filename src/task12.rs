use std::collections::HashSet;

use crate::util::get_task;

const START_ID: &str = "start";
const END_ID: &str = "end";

struct Node<'a> {
    id: &'a str,
    children: Vec<usize>,
}

impl Node<'_> {
    fn is_big(&self) -> bool {
        self.id.chars().all(char::is_uppercase)
    }
}

fn count_paths_internal(graph: &[Node], from: usize, to: usize, path: &mut HashSet<usize>) -> usize {
    if from == to { return 1; }
    let mut count = 0;

    if !graph[from].is_big() { path.insert(from); }
    for child in graph[from].children.iter() {
        if path.contains(child) {
            continue;
        }
        count += count_paths_internal(graph, *child, to, path);
    }
    if !graph[from].is_big() { path.remove(&from); }

    count
}

fn count_paths(graph: &[Node]) -> usize {
    let from = graph.iter().position(|node| *node.id == *START_ID).unwrap();
    let to = graph.iter().position(|node| *node.id == *END_ID).unwrap();
    count_paths_internal(graph, from, to, &mut HashSet::new())
}

fn count_paths_double_internal(graph: &[Node], from: usize, to: usize, path: &mut HashSet<usize>, double_used: Option<usize>) -> usize {
    // same as count_paths, except we can use a small node twice
    if from == to {
        if let Some(used) = double_used {
            return if path.contains(&used) { 1 } else { 0 };
        }
        return 1;
    }
    let mut count = 0;

    // first count the paths if we don't use the node twice
    if !graph[from].is_big() { path.insert(from); }
    for child in graph[from].children.iter() {
        if path.contains(child) { continue; }
        count += count_paths_double_internal(graph, *child, to, path, double_used);
    }
    if !graph[from].is_big() { path.remove(&from); }

    // then count the paths if we do use the node twice
    if double_used.is_none() && !graph[from].is_big() && graph[from].id != START_ID && graph[from].id != END_ID {
        for child in graph[from].children.iter() {
            if path.contains(child) { continue; }
            count += count_paths_double_internal(graph, *child, to, path, Some(from));
        }
    }

    count
}

fn count_paths_double(graph: &[Node]) -> usize {
    let from = graph.iter().position(|node| *node.id == *START_ID).unwrap();
    let to = graph.iter().position(|node| *node.id == *END_ID).unwrap();
    count_paths_double_internal(graph, from, to, &mut HashSet::new(), None)
}

pub fn task12() {
    let start = std::time::Instant::now();
    // io
    let input = get_task(12);
    let edges: Vec<(&str, &str)> = input
        .lines()
        .map(|line| {
            line.split_once('-').unwrap()
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

    // task 1

    let count = count_paths(&graph);
    
    println!("Task 1: {}", count);

    // task 2
    
    let count_with_doubling = count_paths_double(&graph);
    println!("Task 2: {}", count_with_doubling);

    println!("done in {}us!", start.elapsed().as_micros());
}
