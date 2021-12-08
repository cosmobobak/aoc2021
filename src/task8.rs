use crate::get_task::get_task;

fn determine_positionings<'a>(inline: &'a [&str]) -> [&'a str; 10] {
    let seven = *inline.iter().find(|&x| x.len() == 3).unwrap();
    let one = *inline.iter().find(|&x| x.len() == 2).unwrap();
    let four = *inline.iter().find(|&x| x.len() == 4).unwrap();
    let eight = *inline.iter().find(|&x| x.len() == 7).unwrap();
    let top_bar = seven.chars().find(|c| !one.contains(*c)).unwrap();
    let right_chars = one.chars().filter(|c| *c != top_bar).collect::<String>();
    let six = *inline
        .iter()
        .find(|&&s| s.len() == 6 && !right_chars.chars().all(|c| s.contains(c)))
        .unwrap();
    let nine = *inline
        .iter()
        .find(|&&s| {
            s.len() == 6
                && right_chars.chars().all(|c| s.contains(c))
                && s.chars().filter(|&c| four.contains(c)).count() == 4
        })
        .unwrap();
    let zero = *inline
        .iter()
        .find(|&&s| s.len() == 6 && s != six && s != nine)
        .unwrap();
    let three = *inline
        .iter()
        .find(|&&s| s.len() == 5 && s != nine && one.chars().all(|c| s.contains(c)))
        .unwrap();
    let five = *inline
        .iter()
        .find(|&&s| {
            s.len() == 5
                && s != nine
                && s != three
                && four.chars().filter(|&c| s.contains(c)).count() == 3
        })
        .unwrap();
    let two = *inline
        .iter()
        .find(|&&s| {
            s.len() == 5
                && s != nine
                && s != three
                && s != five
                && four.chars().filter(|&c| s.contains(c)).count() == 2
        })
        .unwrap();

    [zero, one, two, three, four, five, six, seven, eight, nine]
}

pub fn task8() {
    let start = std::time::Instant::now();
    // io
    let input = get_task(8);
    let outputs = input.lines().map(|s| s.split(" | ").nth(1).unwrap());
    let outputs = outputs
        .map(|s| s.split_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<_>>();
    let inputs = input.lines().map(|s| s.split(" | ").next().unwrap());
    let inputs = inputs
        .map(|s| s.split_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<_>>();

    // task 1

    let mut nums = vec![0; 9];
    for output in outputs.iter() {
        for num in output {
            match num.len() {
                2 => nums[1] += 1,
                3 => nums[7] += 1,
                4 => nums[4] += 1,
                7 => nums[8] += 1,
                _ => continue,
            }
        }
    }
    println!("Task 1: {}", nums.iter().sum::<usize>());
    // task 2

    let mut sum = 0;
    for (inline, outline) in inputs.iter().zip(outputs.iter()) {
        let nums = determine_positionings(inline);
        let mut value = 0;
        println!("{:?}", outline);
        for l in outline {
            let idx = nums
                .iter()
                .position(|numstr| numstr.len() == l.len() && numstr.chars().all(|c| l.contains(c)))
                .unwrap();
            value *= 10;
            value += idx;
        }
        println!("v: {}", value);
        sum += value;
    }

    println!("Task 2: {}", sum);
    println!("done in {}us!", start.elapsed().as_micros());
}
