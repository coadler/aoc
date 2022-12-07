use itertools::Itertools;
use std::collections::HashSet;

pub fn run(input: &[u8]) -> u32 {
    let mut bt0: HashSet<u8> = HashSet::with_capacity(64);
    let mut bt1: HashSet<u8> = HashSet::with_capacity(64);
    let mut bt2: HashSet<u8> = HashSet::with_capacity(64);

    input
        .split(|b| *b == b'\n')
        .tuples::<(&[u8], &[u8], &[u8])>()
        .map(|bb| {
            for &b in bb.0 {
                bt0.insert(b);
            }
            for &b in bb.1 {
                bt1.insert(b);
            }
            for &b in bb.2 {
                bt2.insert(b);
            }

            let mut c: u32 = 0;
            let intersect1: HashSet<&u8> = bt0.intersection(&bt1).collect();
            let intersect2: HashSet<&u8> = bt0.intersection(&bt2).collect();
            let intersect3: Vec<&&u8> = intersect1.intersection(&intersect2).collect();

            for &b in intersect3 {
                c += if b >= &b'a' {
                    *b - b'a' + 1
                } else {
                    *b - b'A' + 27
                } as u32
            }

            bt0.clear();
            bt1.clear();
            bt2.clear();
            c
        })
        .sum()
}
