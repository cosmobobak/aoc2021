use crate::util::{get_task, mut_median};

fn value(c: char) -> i32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Unknown character"),
    }
}

fn closing(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("Unknown character"),
    }
}

pub fn task10() {
    let start = std::time::Instant::now();
    // io
    let input = get_task(10);
    let lines = input.lines().collect::<Vec<&str>>();

    // task 1
    let mut sum = 0;
    for line in lines.iter() {
        let mut stack = Vec::with_capacity(line.len());
        for brack in line.chars() {
            match brack {
                '(' | '[' | '{' | '<' => stack.push(brack),
                found @ (')' | ']' | '}' | '>') => {
                    if stack.pop().map(closing) != Some(found) {
                        sum += value(found);
                        break;
                    }
                }
                _ => {}
            }
        }
    }
    
    println!("Task 1: {}", sum);

    // task 2
    let incompletes = lines.into_iter().filter(|line| {
        let mut stack = Vec::with_capacity(line.len());
        for brack in line.chars() {
            match brack {
                '(' | '[' | '{' | '<' => stack.push(brack),
                ')' | ']' | '}' | '>' => {
                    if stack.pop().map(closing) != Some(brack) {
                        return false;
                    }
                }
                _ => {}
            }
        }
        true
    }).collect::<Vec<&str>>();

    let mut vals = incompletes.iter().map(|line| {
        let mut stack = Vec::with_capacity(line.len());
        for brack in line.chars() {
            match brack {
                '(' | '[' | '{' | '<' => {stack.push(brack);},
                ')' | ']' | '}' | '>' => {stack.pop();},
                _ => {}
            }
        }
        stack.iter().rev().fold(0, |acc, brack| {
            acc * 5 + match brack {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => panic!("Unknown character"),
            }
        })
    }).collect::<Vec<u64>>();
    let med = *mut_median(&mut vals);

    println!("Task 2: {}", med);

    println!("done in {}us!", start.elapsed().as_micros());
}
