#![allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]

use crate::util::{get_task, mut_median};

pub fn task7() {
    // io
    let input = get_task(7);
    let line = input.lines().next().unwrap();
    let mut nums = line.split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    
    // task 1
    let med = *mut_median(&mut nums);
    let dist = nums.iter().map(|n| {
        (n - med).abs()
    }).sum::<i32>();

    println!("Task 1: {}", dist);
    // task 2
    let mean = f64::from(nums.iter().sum::<i32>()) / nums.len() as f64;
    let x = mean.floor() as i32;
    let y = mean.ceil() as i32;
    let dist_lo = nums.iter().map(|n| {
        let d = (n - x).abs();
        (d * (d + 1)) / 2
    }).sum::<i32>();
    let dist_hi = nums.iter().map(|n| {
        let d = (n - y).abs();
        (d * (d + 1)) / 2
    }).sum::<i32>();
    let dist = std::cmp::min(dist_hi, dist_lo);

    println!("Task 2: {}", dist);
}
