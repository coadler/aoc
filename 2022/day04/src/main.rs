#![feature(portable_simd)]
use core::simd::u8x4;

mod part1;
mod part2;

fn main() {
    let bytes = include_bytes!("../input.txt");
    println!("pt1: {}", part1::run(bytes));
    println!("pt2: {}", part2::run(bytes));
}
