use crate::util::get_task;

fn update(matrix: &mut [[u8; 10]; 10]) -> usize {
    for row in matrix.iter_mut() {
        for cell in row.iter_mut() {
            *cell += 1;
        }
    }

    let mut flashlocs = Vec::with_capacity(100);
    let mut changes = 0;
    let mut change_made = true;
    while change_made {
        change_made = false;
        for i in 0..10 {
            for j in 0..10 {
                if matrix[i][j] > 9 && !flashlocs.contains(&(i, j)) {
                    flashlocs.push((i, j));
                    change_made = true;
                    changes += 1;
                    // update all adjacent cells
                    if i > 0 { matrix[i - 1][j] += 1; }
                    if i < 9 { matrix[i + 1][j] += 1; }
                    if j > 0 { matrix[i][j - 1] += 1; }
                    if j < 9 { matrix[i][j + 1] += 1; }
                    // and corners
                    if i > 0 && j > 0 { matrix[i - 1][j - 1] += 1; }
                    if i > 0 && j < 9 { matrix[i - 1][j + 1] += 1; }
                    if i < 9 && j > 0 { matrix[i + 1][j - 1] += 1; }
                    if i < 9 && j < 9 { matrix[i + 1][j + 1] += 1; }
                }
            }
        }
    }

    for &(i, j) in flashlocs.iter() {
        matrix[i][j] = 0;
    }

    changes
}

pub fn task11() {
    let start = std::time::Instant::now();
    // io
    let input = get_task(11);
    let mut matrix = [[0; 10]; 10];
    for (col, l) in input.lines().enumerate() {
        for (row, c) in l.as_bytes().iter().enumerate() {
            matrix[row][col] = *c - b'0';
        }
    }

    // task 1
    
    let mut flashes = 0;
    let mut matrix_t1 = matrix;
    for _ in 0..100 {
        flashes += update(&mut matrix_t1);
    }
    
    println!("Task 1: {}", flashes);

    // task 2
    
    let mut i: usize = 1;
    let mut matrix_t2 = matrix;
    let all_iter = loop {
        let change_count = update(&mut matrix_t2);
        if change_count == 100 {
            break i;
        }
        i += 1;
    };
    
    println!("Task 2: {}", all_iter);

    println!("done in {}us!", start.elapsed().as_micros());
}
