#![allow(dead_code, unused_imports)]

mod get_task;
mod task1;
mod task2;
mod task3;

use task1::task1;
use task2::task2;
use task3::task3;

fn main() {
    println!("Day 1:");
    task1();
    println!("Day 2:");
    task2();
    println!("Day 3:");
    task3();
}
