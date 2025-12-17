use std::fmt::Display;

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    include_str!("input.txt")
        .trim()
        .bytes()
        .scan(0, |state, b| {
            if b == b'^' {
                *state += 1i64;
            } else {
                *state -= 1;
            }
            Some(*state)
        })
        .max()
        .unwrap()
}

#[inline]
pub fn solve_part2() -> impl Display {
    let mut up = 0;
    let mut down = 0;
    let mut h = 0;

    include_str!("input.txt")
        .trim()
        .bytes()
        .map(|b| {
            match b {
                b'^' => {
                    up += 1;
                    down = 0;
                    h += up;
                }
                _ => {
                    down += 1;
                    up = 0;
                    h -= down;
                }
            }
            h
        })
        .max()
        .unwrap()
}

#[inline]
pub fn solve_part3() -> impl Display {
    let mut up = 0;
    let mut down = 0;
    let mut h = 0;

    let mut x = 0;
    let mut y = 1;
    let mut fibs = vec![0, 1];
    for _ in 0..20 {
        fibs.push(x + y);
        x = y;
        y = *fibs.last().unwrap();
    }

    include_str!("input.txt")
        .trim()
        .bytes()
        .map(|b| {
            match b {
                b'^' => {
                    h -= fibs[up];
                    up += 1;
                    down = 0;
                    h += fibs[up];
                }
                _ => {
                    h += fibs[down];
                    down += 1;
                    up = 0;
                    h -= fibs[down];
                }
            }
            h
        })
        .max()
        .unwrap()
}
