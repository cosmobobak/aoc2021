use std::collections::HashMap;

use crate::get_task::get_task;

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
            mapped_points.insert(p, mapped_points.get(&p).unwrap_or(&0) + 1);
        }
    }

    mapped_points.values().filter(|&x| *x > 1).count()
}

pub fn task5() {
    let start = std::time::Instant::now();
    // io
    let input = get_task(5);
    let points: Vec<((i16, i16), (i16, i16))> = input
        .lines()
        .map(|line| {
            let mut ps = line.split(" -> ").map(|p| {
                let mut p = p.split(',');
                let x = p.next().unwrap().parse::<i16>().unwrap();
                let y = p.next().unwrap().parse::<i16>().unwrap();
                (x, y)
            });
            let p1 = ps.next().unwrap();
            let p2 = ps.next().unwrap();
            (p1, p2)
        })
        .collect();

    // task 1

    let orthos = points
        .iter()
        .filter(|(p1, p2)| p1.0 == p2.0 || p1.1 == p2.1);
    
    println!("Task 1: {}", count_overlaps(orthos));

    // task 2

    println!("Task 2: {}", count_overlaps(points.iter()));

    println!("done in {}us!", start.elapsed().as_micros());
}
