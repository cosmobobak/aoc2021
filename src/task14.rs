use std::collections::HashMap;

use crate::util::get_task;

fn pass(str: &mut HashMap<(char, char), u64>, counts: &mut HashMap<char, u64>, rules: &HashMap<(char, char), char>) {
    let mut new_entries = Vec::with_capacity(20);
    for ((a, b), c) in rules.iter() {
        if let Some(count) = str.remove(&(*a, *b)) {
            new_entries.push(((*a, *c), count));
            new_entries.push(((*c, *b), count));
            counts.entry(*c).and_modify(|e| *e += count).or_insert(count);
        }
    }
    for (k, v) in new_entries {
        str.entry(k).and_modify(|e| *e += v).or_insert(v);
    }
}

fn iterate_pairs(seed: &str, rules: &HashMap<(char, char), char>, runs: u64) -> (u64, u64) {
    let mut pair_counts = HashMap::new();
    for pair in seed.chars().zip(seed.chars().skip(1)) {
        pair_counts.entry(pair).and_modify(|e| *e += 1).or_insert(1);
    }
    let mut char_counts = HashMap::new();
    for c in seed.chars() {
        char_counts.entry(c).and_modify(|e| *e += 1).or_insert(1);
    }
    
    for _ in 0..runs {
        pass(&mut pair_counts, &mut char_counts, rules);
    }
    
    (*char_counts.values().max().unwrap(), *char_counts.values().min().unwrap())
}

pub fn task14() {
    // io
    let input = get_task(14);
    let (seed, rules) = input.split_once("\n\n").unwrap();
    let rules = rules.lines().map(|line| {
        let (a, b) = line.split_once(" -> ").unwrap();
        let mut achars = a.chars();
        ((achars.next().unwrap(), achars.next().unwrap()), b.chars().next().unwrap())
    }).collect::<HashMap<_, _>>();


    // task 1
    let (max_count, min_count) = iterate_pairs(seed, &rules, 10);
    
    println!("Task 1: {}", max_count - min_count);

    // task 2
    let (max_count, min_count) = iterate_pairs(seed, &rules, 40);

    println!("Task 2: {}", max_count - min_count);
}