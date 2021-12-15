use crate::util::get_task;

pub fn task01() {
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
}
