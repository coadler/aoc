use atoi_radix10;

pub fn run(input: &[u8]) -> u32 {
    input[..input.len() - 1]
        .split(|b| *b == b'\n')
        .map(|b| unsafe {
            let mut sp = b.split(|b| *b == b',' || *b == b'-');

            (
                atoi_radix10::parse::<u8>(sp.next().unwrap_unchecked()).unwrap_unchecked(),
                atoi_radix10::parse::<u8>(sp.next().unwrap_unchecked()).unwrap_unchecked(),
                atoi_radix10::parse::<u8>(sp.next().unwrap_unchecked()).unwrap_unchecked(),
                atoi_radix10::parse::<u8>(sp.next().unwrap_unchecked()).unwrap_unchecked(),
            )
        })
        .fold(0, |acc, b| {
            if b.0 <= b.3 && b.2 <= b.1 {
                acc + 1
            } else {
                acc
            }
        })
}
