use std::collections::HashSet;

use crate::util::get_task;

fn do_fold(points: &mut Vec<(i32, i32)>, f: (&str, i32)) {
    let (axis, value) = f;
    points.iter_mut().for_each(|p| {
        match axis {
            "x" => if p.0 > value {
                p.0 = p.0 - (p.0 - value) * 2;
            },
            "y" => if p.1 > value {
                p.1 = p.1 - (p.1 - value) * 2;
            },
            _ => panic!("unknown axis"),
        }
    });
}

fn print_points(points: &[(i32, i32)]) {
    // while this technically has a time complexity of O(n^2),
    // it's likely the most efficient way to print the points
    // as we have a (weak) guarantee that the number of points
    // is a small number.
    let max_x = points.iter().map(|&(x, _)| x).max().unwrap();
    let max_y = points.iter().map(|&(_, y)| y).max().unwrap();
    // points.len() is <= 1 + std::cmp::min(max_x, max_y), which doesn't
    // really have time-complexity implications, but it's nice to know.
    for y in 0..=max_y {
        for x in 0..=max_x {
            if points.contains(&(x, y)) {
                print!("█");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

pub fn task13() {
    let start = std::time::Instant::now();
    // io
    let input = get_task(13);
    let (point_text, fold_text) = input.split_once("\n\n").unwrap();
    let mut points: Vec<(i32, i32)> = point_text
        .lines()
        .map(|line| {
            line.split_once(',').map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap())).unwrap()
        })
        .collect();
    let mut folds = fold_text
        .lines()
        .map(|line| {
            line
                .split(' ')
                .nth(2)
                .unwrap()
                .split_once('=')
                .map(|(axis, value_str)| (axis, value_str.parse::<i32>().unwrap()))
                .unwrap()
        });

    // task 1
    
    let f = folds.next().unwrap();
    do_fold(&mut points, f);
    let unique_points = points.iter().collect::<HashSet<_>>();

    println!("Task 1: {}", unique_points.len());

    // task 2

    for f in folds {
        do_fold(&mut points, f);
    }

    println!("Task 2:");
    print_points(&points);

    println!("done in {}us!", start.elapsed().as_micros());
}