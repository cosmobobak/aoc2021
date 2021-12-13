use std::mem::MaybeUninit;

use crate::util::get_task;

type Bingarr = [[usize; 5]; 5];

fn is_bingo(bingo: &Bingarr, marked: &[usize]) -> bool {
    bingo.iter().any(|r| r.iter().all(|v| marked.contains(v)))
        || (0..bingo.len()).any(|i| bingo.iter().all(|r| marked.contains(&r[i])))
}

fn unmarked_sum(bingo: &Bingarr, marked: &[usize]) -> usize {
    bingo
        .iter()
        .flat_map(|r| r.iter())
        .filter(|v| !marked.contains(v))
        .sum()
}

pub fn task4() {
    // io
    let input = get_task(4);
    let (l1, rest) = input.split_once("\n\n").unwrap();
    let nums: Vec<usize> = l1
        .split(',')
        .map(|v| v.parse().unwrap())
        .collect();
        
    let bingos: Vec<[[usize; 5]; 5]> = rest.split("\n\n")
        .map(|chnk| {
            chnk.lines()
                .map(|l| l.split_whitespace().map(|v| v.parse().unwrap()))
        })
        .map(|bingo| {
            // Create an uninitialized array of `MaybeUninit`. The `assume_init` is
            // safe because the type we are claiming to have initialized here is a
            // bunch of `MaybeUninit`s, which do not require initialization.
            let mut out: [[MaybeUninit<usize>; 5]; 5] = unsafe {
                MaybeUninit::uninit().assume_init()
            };
            for (row, line) in bingo.enumerate() {
                for (col, val) in line.enumerate() {
                    out[row][col].write(val);
                }
            }
            // all initialized, so we can safely convert
            unsafe { std::mem::transmute(out) }
        })
        .collect();

    // task 1

    'outer1: for i in 1..nums.len() {
        let marked = &nums[..i];
        for b in &bingos {
            if is_bingo(b, marked) {
                let res = unmarked_sum(b, marked) * marked.last().unwrap();
                println!("part 1: {}", res);
                break 'outer1;
            }
        }
    }

    // task 2

    let mut won_bingos = vec![false; bingos.len()];
    let mut count = 0;
    'outer2: for i in 1..nums.len() {
        let marked = &nums[..i];
        for (i, b) in bingos.iter().enumerate() {
            if won_bingos[i] {
                continue;
            }
            if is_bingo(b, marked) {
                won_bingos[i] = true;
                count += 1;
            }
            if count == bingos.len() {
                let res = unmarked_sum(b, marked) * marked.last().unwrap();
                println!("part 2: {}", res);
                break 'outer2;
            }
        }
    }
}
