use std::collections::HashMap;
use std::cmp::Reverse;
use priority_queue::PriorityQueue;

use crate::util::get_task;

type Point = (usize, usize);

struct NeighbourIter {
    loc: Point,
    dir: usize,
    width: usize,
    height: usize,
}

impl Iterator for NeighbourIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        match self.dir {
            0 => {
                self.dir += 1;
                if self.loc.0 > 0 { return Some((self.loc.0 - 1, self.loc.1)); };
                self.next()
            }
            1 => {
                self.dir += 1;
                if self.loc.1 > 0 { return Some((self.loc.0, self.loc.1 - 1)); };
                self.next()
            }
            2 => {
                self.dir += 1;
                if self.loc.0 < self.width - 1 { return Some((self.loc.0 + 1, self.loc.1)); };
                self.next()
            }
            _ => {
                if self.loc.1 < self.height - 1 { return Some((self.loc.0, self.loc.1 + 1)); };
                None
            }
        }
    }
}

const fn neighbours(loc: Point, width: usize, height: usize) -> NeighbourIter {
    NeighbourIter {
        width,
        height,
        loc,
        dir: 0,
    }
}


fn dijkstra(graph: &[Vec<u8>], start: Point, end: Point) -> u64 {
    // https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm#Using_a_priority_queue
    let width = graph.len();
    let height = graph[0].len();
    let mut dist = HashMap::new();
    dist.insert(start, 0);

    let mut pq = PriorityQueue::new();

    for (row, vs) in graph.iter().enumerate() {
        for (col, _) in vs.iter().enumerate() {
            if (row, col) != start {
                dist.insert((row, col), std::u64::MAX);
            }
        }
    }

    pq.push(start, Reverse(0));

    while let Some((u, _)) = pq.pop() {
        if u == end { break; }
        for v in neighbours(u, width, height) {
            let alt = dist[&u] + u64::from(graph[v.0][v.1]);
            if alt < dist[&v] {
                dist.insert(v, alt);
                pq.push(v, Reverse(alt));
            }
        }
    }

    dist[&end]
}

fn generate_full_matrix(mut matrix: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    const GROWTH_FACTOR: u8 = 4; // not 5! we only add four copies.
    let rows = matrix.len();
    let cols = matrix[0].len();

    // grow the matrix rightward
    for iteration in 0..GROWTH_FACTOR {
        for row in &mut matrix {
            let new_row = row[0..cols].to_owned();
            for item in new_row {
                row.push((item + iteration) % 9 + 1);
            }
        }
    }

    // grow the matrix downward
    for iteration in 0..GROWTH_FACTOR {
        let new_matrix = matrix[0..rows].to_owned();
        for mut row in new_matrix {
            row.iter_mut().for_each(|x| *x = (*x + iteration) % 9 + 1);
            matrix.push(row);
        }
    }

    matrix
}

pub fn task15() {
    // io
    let input = get_task(15);
    let matrix: Vec<Vec<u8>> = input
        .lines()
        .map(str::as_bytes)
        .map(|row| row.iter().map(|&x| x - b'0').collect())
        .collect();

    // task 1

    let start = (0, 0);
    let end = (matrix.len() - 1, matrix[0].len() - 1);
    let spath = dijkstra(&matrix, start, end);
    
    println!("Task 1: {}", spath);

    // task 2

    let matrix = generate_full_matrix(matrix);
    let start = (0, 0);
    let end = (matrix.len() - 1, matrix[0].len() - 1);
    let spath = dijkstra(&matrix, start, end);

    println!("Task 2: {}", spath);
}