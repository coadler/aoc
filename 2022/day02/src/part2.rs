#![allow(stable_features)]
use std::{hint::unreachable_unchecked, str};

pub fn run(input: &str) -> u32 {
    let input = input.lines();

    input
        .map(|line| {
            let line = line.as_bytes();
            (line[0] - b'A', line[2] - b'X')
        })
        .map(|tup| {
            let choice = match tup.1 {
                0 => match tup.0 {
                    0 => 2,
                    _ => tup.0 - 1,
                },
                1 => tup.0,
                2 => (tup.0 + 1) % 3,
                _ => unsafe { unreachable_unchecked() },
            };

            (choice + 1 + tup.1 * 3) as u32
        })
        .sum::<u32>()
}
