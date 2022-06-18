use std::collections::HashSet;

use crate::util::get_task;
use rayon::prelude::*;

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

fn count_paths_internal(
    graph: &[Node],
    from: usize,
    to: usize,
    path: &mut HashSet<usize>,
    double_used: Option<usize>,
) -> usize {
    // dfs
    if from == to {
        if let Some(used) = double_used {
            return if path.contains(&used) { 1 } else { 0 };
        }
        return 1;
    }
    let origin = &graph[from];
    let is_big = origin.is_big();

    // first count the paths if we don't use the node twice
    // (this is the default case)
    if !is_big {
        path.insert(from);
    }
    // adaptive multithreading for low depth
    let count: usize = if path.len() < 2 {
        origin.children.par_iter().map(|child| {
            if path.contains(child) {
                0
            } else {
                count_paths_internal(graph, *child, to, &mut path.clone(), double_used)
            }
        }).sum()
    } else {
        origin.children.iter().map(|child| {
            if path.contains(child) {
                0
            } else {
                count_paths_internal(graph, *child, to, path, double_used)
            }
        }).sum()
    };
    if !is_big {
        path.remove(&from);
    }

    // then count the paths if we do use the node twice
    let doubling: usize = if double_used.is_none()
        && !is_big
        && origin.id != START_ID
        && origin.id != END_ID
    {
        origin.children.iter().map(|child| {
            if path.contains(child) {
                0
            } else {
                count_paths_internal(graph, *child, to, path, Some(from))
            }
        }).sum()
    } else {
        0
    };

    count + doubling
}

fn count_paths(graph: &[Node], do_doubling: bool) -> usize {
    let from = graph.iter().position(|node| *node.id == *START_ID).unwrap();
    let to = graph.iter().position(|node| *node.id == *END_ID).unwrap();
    let double_used = if do_doubling { None } else { Some(from) };
    count_paths_internal(
        graph,
        from,
        to,
        &mut HashSet::new(),
        double_used,
    )
}

pub fn task12() {
    // io
    let input = get_task(12);
    let edges: Vec<(&str, &str)> = input
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .collect();

    // setup for both

    let mut graph = edges
        .iter()
        .flat_map(|&(from, to)| vec![from, to])
        .collect::<HashSet<_>>()
        .into_iter()
        .map(|id| Node {
            id,
            children: vec![],
        })
        .collect::<Vec<_>>();
    for &(from, to) in &edges {
        let from_node = graph.iter().position(|node| node.id == from).unwrap();
        let to_node = graph.iter().position(|node| node.id == to).unwrap();
        graph[from_node].children.push(to_node);
        graph[to_node].children.push(from_node);
    }
    let graph = graph;

    // task 1

    let count = count_paths(&graph, false);

    println!("Task 1: {}", count);

    // task 2

    let count_with_doubling = count_paths(&graph, true);
    println!("Task 2: {}", count_with_doubling);
}
