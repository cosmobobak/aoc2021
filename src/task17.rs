use itertools::Itertools;

use crate::util::get_task;

#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy)]
struct Rect {
    bottomleft: Point,
    topright: Point,
}

type Vector = Point;

const fn in_target(tgt: Rect, pos: Point) -> bool {
    tgt.bottomleft.x <= pos.x
        && pos.x <= tgt.topright.x
        && tgt.bottomleft.y <= pos.y
        && pos.y <= tgt.topright.y
}

fn run_trajectory(tgt: Rect, mut trajectory: Vector) -> (bool, i32) {
    let mut loc = Point { x: 0, y: 0 };

    let mut max_y = 0;
    loop {
        loc.x += trajectory.x;
        loc.y += trajectory.y;

        max_y = std::cmp::max(max_y, loc.y);

        if in_target(tgt, loc) {
            return (true, max_y);
        }

        trajectory.x -= trajectory.x.signum();
        trajectory.y -= 1;

        if loc.x > tgt.topright.x || loc.y < tgt.bottomleft.y {
            return (false, 0);
        }
    }
}

fn max_traj(tgt: Rect) -> i32 {
    let bottom_bound = std::cmp::min(tgt.bottomleft.y, 0);
    (0..=tgt.topright.x).cartesian_product(bottom_bound..-tgt.bottomleft.y)
        .filter_map(|(x, y)| {
            let (success, max_y) = run_trajectory(tgt, Vector { x, y });
            if success {
                Some(max_y)
            } else {
                None
            }
        })
        .max()
        .unwrap()
}

fn count_trajs(tgt: Rect) -> usize {
    let bottom_bound = std::cmp::min(tgt.bottomleft.y, 0);
    (0..=tgt.topright.x).cartesian_product(bottom_bound..-tgt.bottomleft.y)
        .filter(|&(x, y)| run_trajectory(tgt, Vector { x, y }).0)
        .count()
}

pub fn task17() {
    // io
    let input = get_task(17);
    let x_pos = input.chars().position(|c| c == 'x').unwrap();
    let y_pos = input.chars().position(|c| c == 'y').unwrap();
    let x = &input[x_pos + 2..y_pos - 2];
    let y = &input[y_pos + 2..];
    let (x_min, x_max) = x
        .split_once("..")
        .map(|(l, r)| (l.parse::<i32>().unwrap(), r.parse::<i32>().unwrap()))
        .unwrap();
    let (y_min, y_max) = y
        .split_once("..")
        .map(|(l, r)| (l.parse::<i32>().unwrap(), r.parse::<i32>().unwrap()))
        .unwrap();

    // let area: Rect = ((x_min, y_min), (x_max, y_max));
    let area = Rect {
        bottomleft: Point { x: x_min, y: y_min },
        topright: Point { x: x_max, y: y_max },
    };

    // task 1

    println!("Task 1: {}", max_traj(area));

    // task 2

    println!("Task 2: {}", count_trajs(area));
}