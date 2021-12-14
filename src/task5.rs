use std::collections::HashMap;

use crate::util::get_task;

fn count_overlaps<'a>(points: impl Iterator<Item = &'a((i16, i16), (i16, i16))>) -> usize {
    let mut mapped_points = HashMap::new();
    for (p1, p2) in points {
        let dx = (p2.0 - p1.0).signum();
        let dy = (p2.1 - p1.1).signum();
        let num_points = if p1.0 == p2.0 {
            (p2.1 - p1.1).abs() + 1
        } else {
            (p2.0 - p1.0).abs() + 1
        };
        for i in 0..num_points {
            let p = (p1.0 + i * dx, p1.1 + i * dy);
            // inc count
            mapped_points.entry(p).and_modify(|e| *e += 1).or_insert(1);
        }
    }

    mapped_points.values().filter(|&x| *x > 1).count()
}

pub fn task5() {
    // io
    let input = get_task(5);
    let points: Vec<((i16, i16), (i16, i16))> = input
        .lines()
        .map(|line| {
            let (left_point, right_point) = line.split_once(" -> ").unwrap();
            let (x1, y1) = left_point.split_once(',').map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap())).unwrap();
            let (x2, y2) = right_point.split_once(',').map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap())).unwrap();
            ((x1, y1), (x2, y2))
        })
        .collect();

    // task 1

    let orthos = points
        .iter()
        .filter(|(p1, p2)| p1.0 == p2.0 || p1.1 == p2.1);
    let count = count_overlaps(orthos);
    
    println!("Task 1: {}", count);

    // task 2
    
    let count = count_overlaps(points.iter());
    
    println!("Task 2: {}", count);
}
