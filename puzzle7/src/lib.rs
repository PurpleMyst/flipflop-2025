use std::fmt::Display;

use memoize::memoize;

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    include_str!("input.txt")
        .lines()
        .map(|line| {
            line.split_once(' ')
                .map(|(w, h)| (w.parse::<u64>().unwrap(), h.parse::<u64>().unwrap()))
                .unwrap()
        })
        .map(|(w, h)| bfs2d(0, 0, w - 1, h - 1))
        .sum::<u64>()
}

#[memoize]
fn bfs2d(x: u64, y: u64, u: u64, v: u64) -> u64 {
    if x == u && y == v {
        return 1;
    } else {
        (if x < u { bfs2d(x + 1, y, u, v) } else { 0 }) + (if y < v { bfs2d(x, y + 1, u, v) } else { 0 })
    }
}

#[memoize]
fn bfs3d(x: u64, y: u64, z: u64, u: u64, v: u64) -> u64 {
    if x == u && y == v && z == u {
        return 1;
    } else {
        (if x < u { bfs3d(x + 1, y, z, u, v) } else { 0 })
            + (if y < v { bfs3d(x, y + 1, z, u, v) } else { 0 })
            + (if z < u { bfs3d(x, y, z + 1, u, v) } else { 0 })
    }
}

#[inline]
pub fn solve_part2() -> impl Display {
    include_str!("input.txt")
        .lines()
        .map(|line| {
            line.split_once(' ')
                .map(|(w, h)| (w.parse::<u64>().unwrap(), h.parse::<u64>().unwrap()))
                .unwrap()
        })
        .map(|(w, h)| bfs3d(0, 0, 0, w - 1, h - 1))
        .sum::<u64>()
}

#[inline]
pub fn solve_part3() -> impl Display {
    include_str!("input.txt")
        .lines()
        .map(|line| {
            line.split_once(' ')
                .map(|(w, h)| (w.parse::<u64>().unwrap(), h.parse::<u64>().unwrap()))
                .unwrap()
        })
        .map(|(n, s)| bfs_nd(0, n as u32, s as u32))
        .sum::<u64>()
}

#[memoize]
fn bfs_nd(position: u32, dims: u32, side: u32) -> u64 {
    if (0..dims).all(|i| {
        let mask = 0b111 << (3 * i);
        let component = (position & mask) >> (3 * i);
        component == side - 1
    }) {
        return 1;
    }

    (0..dims)
        .map(|i| {
            let mask = 0b111 << (3 * i);
            let component = (position & mask) >> (3 * i);
            if component < side - 1 {
                bfs_nd(position + (1 << (3 * i)), dims, side)
            } else {
                0
            }
        })
        .sum()
}
