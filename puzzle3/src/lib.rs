use std::{collections::HashMap, fmt::Display};

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    let mut freq: HashMap<&str, usize> = HashMap::new();
    include_str!("input.txt").lines().for_each(|line| {
        *freq.entry(line).or_default() += 1;
    });
    freq.into_iter().max_by_key(|it| it.1).unwrap().0
}

#[inline]
pub fn solve_part2() -> impl Display {
    include_str!("input.txt")
        .lines()
        .filter(|line| {
            let mut it = line.split(',');
            let r = it.next().unwrap().parse::<u8>().unwrap();
            let g = it.next().unwrap().parse::<u8>().unwrap();
            let b = it.next().unwrap().parse::<u8>().unwrap();
            if r == g || r == b || g == b {
                return false;
            }

            g > r && g > b
        })
        .count()
}

#[inline]
pub fn solve_part3() -> impl Display {
    include_str!("input.txt")
        .lines()
        .map(|line| {
            let mut it = line.split(',');
            let r = it.next().unwrap().parse::<u8>().unwrap();
            let g = it.next().unwrap().parse::<u8>().unwrap();
            let b = it.next().unwrap().parse::<u8>().unwrap();
            let max = r.max(g).max(b);
            if r == g || r == b || g == b {
                10
            } else if max == r {
                5
            } else if max == g {
                2
            } else {
                4
            }
        })
        .sum::<u64>()
}

