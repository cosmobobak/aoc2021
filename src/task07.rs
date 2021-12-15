#![allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]

use crate::util::{get_task, median_element};

pub fn task07() {
    // io
    let input = get_task(7);
    let line = input.lines().next().unwrap();
    let mut crab_positions: Vec<i32> = line.split(',').map(|x| x.parse().unwrap()).collect();
    
    // task 1
    let chosen_location = *median_element(&mut crab_positions);
    let fuel_cost = crab_positions.iter().map(|n| {
        (n - chosen_location).abs()
    }).sum::<i32>();

    println!("Task 1: {}", fuel_cost);
    // task 2
    let mean = f64::from(crab_positions.iter().sum::<i32>()) / crab_positions.len() as f64;
    let x = mean.floor() as i32;
    let y = mean.ceil() as i32;
    let dist_lo = crab_positions.iter().map(|n| {
        let d = (n - x).abs();
        (d * (d + 1)) / 2
    }).sum::<i32>();
    let dist_hi = crab_positions.iter().map(|n| {
        let d = (n - y).abs();
        (d * (d + 1)) / 2
    }).sum::<i32>();
    let fuel_cost = std::cmp::min(dist_hi, dist_lo);

    println!("Task 2: {}", fuel_cost);
}
