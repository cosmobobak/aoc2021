#![allow(dead_code, unused_imports)]

mod get_task;
mod task1;
mod task2;
mod task3;
mod task4;
mod task5;

use task1::task1;
use task2::task2;
use task3::task3;
use task4::task4;
use task5::task5;

fn main() {
    println!("Day 1:");
    task1();
    println!("Day 2:");
    task2();
    println!("Day 3:");
    task3();
    println!("Day 4:");
    task4();
    println!("Day 5:");
    task5();
}
