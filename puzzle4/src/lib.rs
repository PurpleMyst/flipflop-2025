use std::{fmt::Display, iter::once};

use itertools::Itertools;

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    once((0, 0))
        .chain(include_str!("input.txt").lines().map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse::<u16>().unwrap(), y.parse::<u16>().unwrap())
        }))
        .tuple_windows()
        .map(|(a, b)| a.0.abs_diff(b.0) + (a.1.abs_diff(b.1)))
        .sum::<u16>()
}

#[inline]
pub fn solve_part2() -> impl Display {
    once((0, 0))
        .chain(include_str!("input.txt").lines().map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse::<u16>().unwrap(), y.parse::<u16>().unwrap())
        }))
        .tuple_windows()
        .map(|(a, b)| a.0.abs_diff(b.0).max(a.1.abs_diff(b.1)))
        .sum::<u16>()
}

#[inline]
pub fn solve_part3() -> impl Display {
    once((0, 0))
        .chain(include_str!("input.txt").lines().map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse::<u16>().unwrap(), y.parse::<u16>().unwrap())
        }))
        .sorted_unstable_by_key(|p| p.0 + p.1)
        .tuple_windows()
        .map(|(a, b)| a.0.abs_diff(b.0).max(a.1.abs_diff(b.1)))
        .sum::<u16>()
}
