use crate::get_task::get_task;

fn adjvec(lines: &[&[u8]], row: usize, col: usize) -> [u8; 4] {
    let h = lines.len() - 1;
    let w = lines[0].len() - 1;
    [
        if col > 0 { lines[row][col - 1] - b'0' } else { 10 },
        if col < w { lines[row][col + 1] - b'0' } else { 10 },
        if row > 0 { lines[row - 1][col] - b'0' } else { 10 },
        if row < h { lines[row + 1][col] - b'0' } else { 10 },
    ]
}

fn basin_size(lines: &[&[u8]], row: usize, col: usize, seen: &mut Vec<(usize, usize)>) -> usize {
    let h = lines.len() - 1;
    let w = lines[0].len() - 1;
    let local_height = lines[row][col] - b'0';
    if local_height == 9 || seen.contains(&(row, col)) {
        return 0;
    }
    let mut acc = 1;
    if col > 0 && lines[row][col - 1] - b'0' > local_height {
        acc += basin_size(lines, row, col - 1, seen);
    }
    if col < w && lines[row][col + 1] - b'0' > local_height {
        acc += basin_size(lines, row, col + 1, seen);
    }
    if row > 0 && lines[row - 1][col] - b'0' > local_height {
        acc += basin_size(lines, row - 1, col, seen);
    }
    if row < h && lines[row + 1][col] - b'0' > local_height {
        acc += basin_size(lines, row + 1, col, seen);
    }
    seen.push((row, col));
    acc
}

pub fn task9() {
    let start = std::time::Instant::now();
    // io
    let input = get_task(9);
    let lines = input.lines().map(str::as_bytes).collect::<Vec<_>>();

    // task 1
    let mut sum = 0;
    for row in 0..lines.len() {
        for col in 0..lines[0].len() {
            let v = lines[row][col] - b'0';
            let adjs = adjvec(&lines, row, col);
            if adjs.iter().all(|x| *x > v) {
                sum += 1 + v as usize;
            }
        }
    }

    println!("Task 1: {}", sum);

    // task 2
    let mut basins = [0; 3];
    for row in 0..lines.len() {
        for col in 0..lines[0].len() {
            let v = lines[row][col] - b'0';
            let adjs = adjvec(&lines, row, col);
            if adjs.into_iter().all(|x| x > v) {
                let b = basin_size(&lines, row, col, &mut vec![]);
                if basins.iter().any(|x| *x < b) {
                    *basins.iter_mut().min().unwrap() = b;
                }
            }
        }
    }
    let prod = basins.iter().product::<usize>();

    println!("Task 2: {}", prod);

    println!("done in {}us!", start.elapsed().as_micros());
}
