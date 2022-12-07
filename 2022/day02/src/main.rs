#![allow(stable_features)]
#![feature(map_first_last)]

use std::str;

mod part1;
mod part2;

fn main() {
    let input = unsafe { str::from_utf8_unchecked(include_bytes!("../input.txt")) };
    println!("pt1: {}", part1::run(input));
    println!("pt2: {}", part2::run(input));
}
