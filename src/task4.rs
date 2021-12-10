use crate::util::get_task;
use itertools::Itertools;

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
    let start = std::time::Instant::now();
    // io
    let input = get_task(4);
    let mut lines = input.lines();
    let nums: Vec<usize> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|v| v.parse().unwrap())
        .collect();
    lines.next();
    let chunks = lines.chunks(6);
    let bingos: Vec<[[usize; 5]; 5]> = chunks
        .into_iter()
        .map(|chnk| {
            chnk.take(5)
                .map(|l| l.split_whitespace().map(|v| v.parse().unwrap()))
        })
        .map(|bingo| {
            let mut out = [[0; 5]; 5];
            for (row, line) in bingo.enumerate() {
                for (col, val) in line.enumerate() {
                    out[row][col] = val;
                }
            }
            out
        })
        .collect();

    // task 1

    'outer1: for i in 1..nums.len() {
        let marked = &nums[..i];
        for b in bingos.iter() {
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

    println!("done in {}us!", start.elapsed().as_micros());
}
