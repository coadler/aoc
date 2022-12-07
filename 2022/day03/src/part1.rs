use std::collections::HashSet;

pub fn run(input: &[u8]) -> u32 {
    let mut bt0: HashSet<u8> = HashSet::new();
    let mut bt1: HashSet<u8> = HashSet::new();

    input
        .split(|b| *b == b'\n')
        .map(|b| b.split_at(b.len() / 2))
        .map(|bb| {
            for &b in bb.0 {
                bt0.insert(b);
            }

            for &b in bb.1 {
                bt1.insert(b);
            }

            let mut c: u32 = 0;
            let intersect = bt0.intersection(&bt1).collect::<Vec<&u8>>();
            for b in intersect {
                c += if b >= &b'a' {
                    *b - b'a' + 1
                } else {
                    *b - b'A' + 27
                } as u32
            }

            bt0.clear();
            bt1.clear();
            c
        })
        .sum()
}
