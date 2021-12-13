use crate::util::get_task;

pub fn task2() {
    let input = get_task(2);
    let instructions = input
        .lines()
        .map(|string| {
            let (instruction, value) = string.split_once(" ").unwrap();
            let value = value.parse::<i32>().unwrap();
            (instruction, value)
        })
        .collect::<Vec<_>>();

    // task 1

    let mut depth = 0;
    let mut forward = 0;

    for (instruction, value) in &instructions {
        match *instruction {
            "forward" => forward += value,
            "down" => depth += value,
            "up" => depth -= value,
            _ => panic!("Unknown instruction: {}", instruction),
        }
    }

    println!("Task 1: {}", forward * depth);

    // task 2

    let mut depth = 0;
    let mut forward = 0;
    let mut aim = 0;

    for (instruction, value) in &instructions {
        match *instruction {
            "forward" => {
                forward += value;
                depth += value * aim;
            }
            "down" => aim += value,
            "up" => aim -= value,
            _ => panic!("Unknown instruction: {}", instruction),
        }
    }

    println!("Task 2: {}", forward * depth);
}
