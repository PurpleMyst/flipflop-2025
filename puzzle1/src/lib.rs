use std::fmt::Display;

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    include_str!("input.txt")
        .lines()
        .map(|line| line.len() / 2)
        .sum::<usize>()
}

#[inline]
pub fn solve_part2() -> impl Display {
    include_str!("input.txt")
        .lines()
        .map(|line| line.len() / 2)
        .filter(|n| n % 2 == 0)
        .sum::<usize>()
}

#[inline]
pub fn solve_part3() -> impl Display {
    include_str!("input.txt")
        .lines()
        .filter(|line| !line.contains('e'))
        .map(|line| line.len() / 2)
        .sum::<usize>()
}
