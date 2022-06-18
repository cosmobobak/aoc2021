use std::{fmt::{Display, Formatter}, ops::Neg};

use itertools::Itertools;

use crate::util::get_task;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Content {
    Literal(i64),
    Subpair(Box<SFNumber>),
}

impl Display for Content {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Content::Literal(i) => write!(f, "{}", i),
            Content::Subpair(pair) => write!(f, "{}", pair),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SFNumber {
    left: Content,
    right: Content,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Side {
    Left,
    Right,
}

type Path = Vec<Side>;

type PathSlice<'a> = &'a[Side];

impl Neg for Side {
    type Output = Self;

    fn neg(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

impl Display for SFNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.left, self.right)
    }
}

impl SFNumber {
    fn from_str(s: &str) -> Self {
        // [4,[0,8]] -> Pair { left: Content::Literal(4), right: Content::Subpair(Box::new(Pair { left: Content::Literal(0), right: Content::Literal(8) })) }

        let mut bracket_depth = 0;
        let split_point = s.chars().enumerate().filter(|(_, c)| {
            match c {
                '[' => bracket_depth += 1,
                ']' => bracket_depth -= 1,
                _ => (),
            }
            bracket_depth == 1
        }).find(|(_, c)| *c == ',').unwrap().0;

        let left = &s[1..split_point];
        let right = &s[split_point + 1..s.len() - 1];

        Self {
            left: match left.parse::<i64>() {
                Ok(i) => Content::Literal(i),
                Err(_) => Content::Subpair(Box::new(Self::from_str(left))),
            },
            right: match right.parse::<i64>() {
                Ok(i) => Content::Literal(i),
                Err(_) => Content::Subpair(Box::new(Self::from_str(right))),
            },
        }
    }
}

fn split(c: &mut Content) -> bool {
    match c {
        Content::Literal(x) => {
            if *x < 10 {
                return false;
            }
            *c = Content::Subpair(Box::new(SFNumber {
                left: Content::Literal(*x / 2),
                right: Content::Literal(*x / 2 + (*x & 1)),
            }));
        }
        Content::Subpair(_) => {
            panic!("Cannot split subpair");
        }
    }
    true
}

fn do_split(root: &mut SFNumber) -> bool {
    match &mut root.left {
        Content::Literal(_) => {
            let res = split(&mut root.left);
            if res {
                return true;
            }
        }
        Content::Subpair(p) => {
            let res = do_split(&mut *p);
            if res {
                return true;
            }
        }
    }

    match &mut root.right {
        Content::Literal(_) => {
            let res = split(&mut root.right);
            if res {
                return true;
            }
        }
        Content::Subpair(p) => {
            let res = do_split(&mut *p);
            if res {
                return true;
            }
        }
    }

    false
}

fn find_explode_target_inner(root: &SFNumber, d: usize, mut path: Path) -> Option<Path> {
    if d == 4 {
        return Some(path);
    }

    // try left side
    match &root.left {
        Content::Subpair(p) => {
            path.push(Side::Left);
            if let Some(next) = find_explode_target_inner(&*p, d + 1, path.clone()) {
                return Some(next);
            }
            path.pop();
        }
        Content::Literal(_) => {
            // do nothing
        }
    }

    // try right side
    match &root.right {
        Content::Subpair(p) => {
            path.push(Side::Right);
            return find_explode_target_inner(&*p, d + 1, path.clone());
        }
        Content::Literal(_) => {
            // do nothing
        }
    }

    None
}

fn find_explode_target(root: &SFNumber) -> Option<Path> {
    find_explode_target_inner(root, 0, Path::new())
}

fn adjacent(mut p: Path, s: Side) -> Option<Path> {
    let mut popped = 0;
    while let Some(side) = p.pop() {
        if side != s {
            p.push(s);
            for _ in 0..=popped {
                p.push(-s);
            }
            return Some(p);
        }
        popped += 1;
    }
    None
}

fn add_to_node(root: &mut SFNumber, path: PathSlice, v: i64) {
    let mut outer = Some(root);
    for side in path {
        let mut p = outer.take().unwrap();
        if *side == Side::Left {
            match &mut p.left {
                Content::Subpair(next) => p = next,
                Content::Literal(lit) => {
                    *lit += v;
                    return;
                },
            }
        } else {
            match &mut p.right {
                Content::Subpair(next) => p = next,
                Content::Literal(lit) => {
                    *lit += v;
                    return;
                },
            }
        }
        outer = Some(p);
    }
}

fn get_value(root: &SFNumber, mut path: Path, side: Side) -> i64 {
    if path.is_empty() {
        return match side {
            Side::Left => {
                match &root.left {
                    Content::Literal(x) => *x,
                    Content::Subpair(_) => panic!("Cannot get value of subpair"),
                }
            }
            Side::Right => {
                match &root.right {
                    Content::Literal(x) => *x,
                    Content::Subpair(_) => panic!("Cannot get value of subpair"),
                }
            }
        };
    }
    let decision = path.remove(0);
    let new_node = match decision {
        Side::Left => {
            match &root.left {
                Content::Subpair(p) => p,
                Content::Literal(_) => panic!("Cannot get value of literal"),
            }
        }
        Side::Right => {
            match &root.right {
                Content::Subpair(p) => p,
                Content::Literal(_) => panic!("Cannot get value of literal"),
            }
        }
    };
    get_value(new_node, path, side)
}

fn zero_out(root: &mut SFNumber, mut path: Path) {
    if path.len() == 1 {
        let decision = path[0];
        if decision == Side::Left {
            root.left = Content::Literal(0);
        } else {
            root.right = Content::Literal(0);
        }
        return;
    }
    let decision = path.remove(0);
    let new_node = match decision {
        Side::Left => {
            match &mut root.left {
                Content::Subpair(p) => p,
                Content::Literal(_) => panic!("Cannot get value of literal"),
            }
        }
        Side::Right => {
            match &mut root.right {
                Content::Subpair(p) => p,
                Content::Literal(_) => panic!("Cannot get value of literal"),
            }
        }
    };
    zero_out(new_node, path);
}

fn do_explode(root: &mut SFNumber) -> bool {
    match find_explode_target(root) {
        Some(path) => {
            let left_val = get_value(root, path.clone(), Side::Left);
            let right_val = get_value(root, path.clone(), Side::Right);

            zero_out(root, path.clone());

            if let Some(left_path) = adjacent(path.clone(), Side::Left) {
                add_to_node(root, &left_path, left_val);
            }
            
            if let Some(right_path) = adjacent(path, Side::Right) {
                add_to_node(root, &right_path, right_val);
            }
            
            true
        }
        None => {
            false
        }
    }
}

fn reduce(root: &mut SFNumber) {
    loop {
        let res1 = do_explode(root);
        if res1 {
            continue;
        }

        let res2 = do_split(root);
        
        if !res1 && !res2 {
            break;
        }
    }
}

fn pair_add(a: SFNumber, b: SFNumber) -> SFNumber {
    let mut c = SFNumber {
        left: Content::Subpair(Box::new(a)),
        right: Content::Subpair(Box::new(b)),
    };
    reduce(&mut c);
    c
}

fn magnitude(p: &SFNumber) -> i64 {
    let mag_l = match &p.left {
        Content::Subpair(p) => magnitude(p),
        Content::Literal(x) => *x,
    };

    let mag_r = match &p.right {
        Content::Subpair(p) => magnitude(p),
        Content::Literal(x) => *x,
    };

    3 * mag_l + 2 * mag_r
}

pub fn task18() {
    // io
    let input = get_task(18);

    // task 1

    let sum = input.lines().map(SFNumber::from_str).reduce(pair_add).unwrap();

    let mag = magnitude(&sum);

    println!("Task 1: {}", mag);

    // task 2

    let max_sum = input
        .lines()
        .map(SFNumber::from_str)
        .cartesian_product(input
            .lines()
            .map(SFNumber::from_str))
        .filter(|(a, b)| a != b)
        .map(|(a, b)| pair_add(a, b))
        .map(|sfn| magnitude(&sfn))
        .max()
        .unwrap();

    println!("Task 2: {}", max_sum);
}
