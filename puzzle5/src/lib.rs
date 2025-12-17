use std::{
    collections::{HashMap, hash_map::Entry},
    fmt::Display,
};

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    let mut seen = HashMap::new();
    let mut tunnels = HashMap::new();
    let len = include_str!("input.txt").trim().len();

    include_str!("input.txt")
        .trim()
        .bytes()
        .enumerate()
        .for_each(|(i, b)| match seen.entry(b) {
            Entry::Occupied(occupied_entry) => {
                let &j = occupied_entry.get();
                tunnels.insert(i, j);
                tunnels.insert(j, i);
            }
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(i);
            }
        });

    let mut i = 0usize;
    let mut steps = 0;
    while i < len {
        let j = *tunnels.get(&i).unwrap();
        steps += i.abs_diff(j);
        i = j + 1;
    }
    steps
}

#[inline]
pub fn solve_part2() -> impl Display {
    let mut seen = HashMap::new();
    let mut tunnels = HashMap::new();
    let len = include_str!("input.txt").trim().len();
    let mut visited = vec![false; len];

    include_str!("input.txt")
        .trim()
        .bytes()
        .enumerate()
        .for_each(|(i, b)| match seen.entry(b) {
            Entry::Occupied(occupied_entry) => {
                let &j = occupied_entry.get();
                tunnels.insert(i, j);
                tunnels.insert(j, i);
            }
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(i);
            }
        });

    let mut i = 0usize;
    while i < len {
        visited[i] = true;
        let j = *tunnels.get(&i).unwrap();
        visited[j] = true;
        i = j + 1;
    }
    include_str!("input.txt")
        .trim()
        .bytes()
        .enumerate()
        .filter_map(|(i, b)| (!visited[i] && i < tunnels[&i]).then_some(b as char))
        .collect::<String>()
}

#[inline]
pub fn solve_part3() -> impl Display {
    let mut seen = HashMap::new();
    let mut tunnels = HashMap::new();

    let len = include_str!("input.txt").trim().len();
    let neg = include_str!("input.txt")
        .trim()
        .bytes()
        .map(|b| b.is_ascii_uppercase())
        .collect::<Vec<bool>>();

    include_str!("input.txt")
        .trim()
        .bytes()
        .enumerate()
        .for_each(|(i, b)| match seen.entry(b) {
            Entry::Occupied(occupied_entry) => {
                let &j = occupied_entry.get();
                tunnels.insert(i, j);
                tunnels.insert(j, i);
            }
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(i);
            }
        });

    let mut i = 0usize;
    let mut steps = 0isize;
    while i < len {
        let j = *tunnels.get(&i).unwrap();
        if neg[i] {
            steps -= i.abs_diff(j) as isize;
        } else {
            steps += i.abs_diff(j) as isize;
        }
        i = j + 1;
    }
    steps
}
