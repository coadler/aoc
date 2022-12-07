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
            let outcome = if (tup.0 + 1) % 3 == tup.1 {
                6u8
            } else if tup.0 == tup.1 {
                3u8
            } else {
                0u8
            };

            (tup.1 + 1 + outcome) as u32
        })
        .sum::<u32>()
}
