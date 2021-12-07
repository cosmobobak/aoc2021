use crate::get_task::get_task;

pub fn task1() {
    let start = std::time::Instant::now();
    // io
    let input = get_task(1);
    let nums = input
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    // task 1
    let sum = nums.windows(2).filter(|w| w[0] < w[1]).count();
    println!("Task 1: {}", sum);

    // task 2
    let windows = nums
        .windows(3)
        .map(|w| w.iter().sum())
        .collect::<Vec<i32>>();
    let sum = windows.windows(2).filter(|w| w[0] < w[1]).count();
    println!("Task 2: {}", sum);
    println!("done in {}us!", start.elapsed().as_micros());
}
