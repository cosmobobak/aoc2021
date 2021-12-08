use adqselect::nth_element;

use crate::get_task::get_task;

pub fn task7() {
    let start = std::time::Instant::now();
    // io
    let input = get_task(7);
    let line = input.lines().next().unwrap();
    let mut nums = line.split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    
    // task 1
    let idx = nums.len() / 2;
    nth_element(&mut nums, idx, &mut Ord::cmp);
    let x = nums[idx];
    let dist = nums.iter().map(|n| {
        (n - x).abs()
    }).sum::<i32>();

    println!("Task 1: {}", dist);
    // task 2
    let mean = nums.iter().sum::<i32>() as f64 / nums.len() as f64;
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
    println!("done in {}us!", start.elapsed().as_micros());
}
