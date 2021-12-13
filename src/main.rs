#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
)]

mod util;
mod task1;
mod task2;
mod task3;
mod task4;
mod task5;
mod task6;
mod task7;
mod task8;
mod task9;
mod task10;
mod task11;
mod task12;
mod task13;

use task1::task1;
use task2::task2;
use task3::task3;
use task4::task4;
use task5::task5;
use task6::task6;
use task7::task7;
use task8::task8;
use task9::task9;
use task10::task10;
use task11::task11;
use task12::task12;
use task13::task13;

static mut DAY_COUNTER: i32 = 1;

fn run(day: impl FnOnce()) -> u128 {
    println!("Day {}:", unsafe { DAY_COUNTER });
    unsafe { DAY_COUNTER += 1 };
    let start = std::time::Instant::now();
    day();
    start.elapsed().as_millis()
}

fn main() {
    let mut timings = Vec::new();
    let start = std::time::Instant::now();
    timings.push(run(task1));
    timings.push(run(task2));
    timings.push(run(task3));
    timings.push(run(task4));
    timings.push(run(task5));
    timings.push(run(task6));
    timings.push(run(task7));
    timings.push(run(task8));
    timings.push(run(task9));
    timings.push(run(task10));
    timings.push(run(task11));
    timings.push(run(task12));
    timings.push(run(task13));
    println!("Total time for all days: {}ms", start.elapsed().as_millis());
    println!("Timings:");
    for (i, timing) in timings.iter().enumerate() {
        println!("Day {}: {}ms", i + 1, timing);
    }
}
