pub fn run(input: &str) -> u32 {
    let input = input.lines();

    let mut vec: Vec<u32> = Vec::new();
    let mut last_sum: u32 = 0;
    for line in input {
        if line.len() == 0 {
            vec.push(last_sum);
            last_sum = 0;
            continue;
        }

        last_sum += unsafe { line.parse::<u32>().unwrap_unchecked() };
    }

    vec.sort_by(|a, b| b.cmp(a));
    vec.iter().take(3).sum()
}
