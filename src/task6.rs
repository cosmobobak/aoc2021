use rug::Integer;

use crate::util::get_task;

fn pass(ages: &mut [Integer]) {
    ages.rotate_left(1);
    let (left, right) = ages.split_at_mut(8);
    left[6] += &right[0];
}

fn count_at_step(data: &[usize], iteration: usize) -> Integer {
    let mut ages = vec![Integer::with_capacity(64 * 100); 9];
    for &d in data.iter() {
        ages[d] += 1;
    }
    for _ in 0..iteration {
        pass(&mut ages);
    }
    ages.iter().sum()
}

pub fn task6() {
    let start = std::time::Instant::now();
    // io
    let input = get_task(6);
    let data: Vec<usize> = input
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    
    // task 1

    let count = count_at_step(&data, 80);

    println!("Task 1: {:?}", count);

    // task 2
    
    let count = count_at_step(&data, 256);
    
    println!("Task 2: {:?}", count);

    println!("done in {}us!", start.elapsed().as_micros());

    // let time_for_ten_million = std::time::Instant::now();
    // let res = count_at_step(&data, 10_000_000);
    // let time_for_ten_million = time_for_ten_million.elapsed().as_micros();
    // println!("{}", res);
    // println!("done in {}us!", time_for_ten_million);
}
