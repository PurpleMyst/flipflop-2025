use std::fmt::Display;

use rayon::prelude::*;

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    include_str!("input.txt")
        .lines()
        .map(|line| {
            line.split_once(',')
                .map(|(x, y)| (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap()))
                .unwrap()
        })
        .map(|(vx, vy)| ((100 * vx).rem_euclid(1000), (100 * vy).rem_euclid(1000)))
        .filter(|&(x, y)| x >= 250 && x < 750 && y >= 250 && y < 750)
        .count()
}

#[inline]
pub fn solve_part2() -> impl Display {
    let birds = include_str!("input.txt")
        .lines()
        .map(|line| {
            line.split_once(',')
                .map(|(x, y)| (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap()))
                .unwrap()
        })
        .collect::<Vec<(i64, i64)>>();

    (1..=1000)
        .map(|h| {
            birds
                .iter()
                .map(|(vx, vy)| {
                    (
                        ((3600 % 1000) * h * vx).rem_euclid(1000),
                        ((3600 % 1000) * h * vy).rem_euclid(1000),
                    )
                })
                .filter(|&(x, y)| x >= 250 && x < 750 && y >= 250 && y < 750)
                .count()
        })
        .sum::<usize>()
}

#[inline]
pub fn solve_part3() -> impl Display {
    let birds = include_str!("input.txt")
        .lines()
        .map(|line| {
            line.split_once(',')
                .map(|(x, y)| (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap()))
                .unwrap()
        })
        .collect::<Vec<(i64, i64)>>();

    (1..=1000)
        .into_par_iter()
        .map(|h| {
            birds.iter()
                .map(|(vx, vy)| {
                    (
                        ((31556926 % 1000) * h * vx).rem_euclid(1000),
                        ((31556926 % 1000) * h * vy).rem_euclid(1000),
                    )
                })
                .filter(|&(x, y)| x >= 250 && x < 750 && y >= 250 && y < 750)
                .count()
        })
        .sum::<usize>()
}
