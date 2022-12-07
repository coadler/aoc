use arrayvec::ArrayVec;
use atoi_radix10;
use std::collections::VecDeque;
use std::iter;
use std::ops::IndexMut;

pub fn run(input: &[u8]) -> String {
    let mut matrix: ArrayVec<VecDeque<char>, 32> =
        ArrayVec::from_iter::<_>(iter::repeat(VecDeque::with_capacity(32)).take(16));

    input[..input.len() - 1]
        .split(|b| *b == b'\n')
        .take_while(|&b| !b.starts_with(b" 1"))
        .for_each(|b| {
            b.iter()
                .skip(1)
                .enumerate()
                .filter(|b| b.0 % 4 == 0 && *b.1 >= b'A' && *b.1 <= b'Z')
                .map(|b| ((b.0 / 4) as usize, b.1))
                .for_each(|b| {
                    matrix.index_mut(b.0).push_front(*b.1 as char);
                });
        });

    input[..input.len() - 1]
        .split(|b| *b == b'\n')
        .skip_while(|&b| !b.starts_with(b"move"))
        .enumerate()
        .for_each(|b| unsafe {
            let mut sp = b.1.split(|b| *b == b' ');

            sp.next();
            let count: u8 = atoi_radix10::parse(sp.next().unwrap_unchecked()).unwrap_unchecked();
            sp.next();
            let from: usize =
                atoi_radix10::parse::<usize>(sp.next().unwrap_unchecked()).unwrap_unchecked() - 1;
            sp.next();
            let to: usize =
                atoi_radix10::parse::<usize>(sp.next().unwrap_unchecked()).unwrap_unchecked() - 1;

            let from = matrix.index_mut(from);
            let mut v = from.split_off(from.len() - count as usize);
            matrix.index_mut(to).append(&mut v);
        });

    let mut s = String::with_capacity(16);
    for m in matrix.iter_mut() {
        if let Some(v) = m.pop_back() {
            s.push(v);
        }
    }

    s
}
