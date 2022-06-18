use crate::util::get_task;

use partition::partition_index;

pub fn task03() {
    let input = get_task(3);
    let mut input = input
        .lines()
        .map(|b| b.chars().rev().collect::<String>())
        .map(|x| usize::from_str_radix(&x, 2).unwrap())
        .collect::<Vec<_>>();

    // task 1

    let mut val: usize = 0;
    for col in 0..12 {
        let mut sum = 0;
        for v in &input {
            sum += (v >> col) & 1;
        }
        val <<= 1;
        val += if sum > input.len() / 2 { 1 } else { 0 };
    }

    let gamma = val;
    let epsilon = !val & 0b1111_1111_1111;

    println!("Task 1: {}", gamma * epsilon);

    // task 2

    let flip_bits = |i: usize| -> usize {
        let mut res = 0;
        for j in 0..12 {
            res <<= 1;
            res += (i >> j) & 1;
        }
        res
    };

    let mut codes = &mut input[..];
    let mut bit = 0;
    let result = loop {
        let bcount = codes.iter().fold(0, |acc, x| acc + ((x >> bit) & 1));
        let most_common = usize::from(bcount * 2 >= codes.len());
        let idx = partition_index(codes, |x| ((x >> bit) & 1) == most_common);
        codes = codes.split_at_mut(idx).0;
        if let [x] = codes {
            break *x;
        }
        bit += 1;
    };

    // deal with the stupid way around that the bits are
    let o2 = flip_bits(result);

    let mut codes = &mut input[..];
    let mut bit = 0;
    let result = loop {
        let bcount = codes.iter().fold(0, |acc, x| acc + ((x >> bit) & 1));
        let least_common = usize::from(bcount * 2 < codes.len());
        let idx = partition_index(codes, |x| ((x >> bit) & 1) == least_common);
        codes = codes.split_at_mut(idx).0;
        if let [x] = codes {
            break *x;
        }
        bit += 1;
    };

    // deal with the stupid way around that the bits are for a second time >:(
    let co2 = flip_bits(result);

    println!("Task 2: {}", o2 * co2);
}
