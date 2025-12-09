use std::fs;

pub fn part1() -> std::io::Result<()> {
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    let content = fs::read_to_string("data/day02.txt")?;
    for line in content.split(',') {
        let mut parts = line.split('-');
        let min: u64 = parts.next().unwrap().parse().unwrap();
        let max: u64 = parts.next().unwrap().parse().unwrap();
        ranges.push((min,max));
    }

    let max_value = ranges.iter().map(|(_, max)| *max).max().unwrap();
    let max_digits = max_value.to_string().len();

    let mut all_doubled = Vec::new();
    for digits in (2..=max_digits).step_by(2) {
        all_doubled.extend(generate_doubled_numbers_1(digits as u32));
    }

    let answer: u64 = all_doubled
        .into_iter()
        .filter(|n| ranges.iter().any(|(min, max)| *n >= *min && *n <= *max))
        .sum();

    println!("Solution for day 2 part 1:: {answer}");

    Ok(())
}

pub fn part2() -> std::io::Result<()> {
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    let content = fs::read_to_string("data/day02.txt")?;
    for line in content.split(',') {
        let mut parts = line.split('-');
        let min: u64 = parts.next().unwrap().parse().unwrap();
        let max: u64 = parts.next().unwrap().parse().unwrap();
        ranges.push((min,max));
    }

    let max_value = ranges.iter().map(|(_, max)| *max).max().unwrap();
    let max_digits = max_value.to_string().len();

    let mut all_doubled = Vec::new();
    for digits in 2..=max_digits {
        all_doubled.extend(generate_doubled_numbers_2(digits as u32));
    }

    all_doubled.sort();
    all_doubled.dedup();

    let answer: u64 = all_doubled
        .into_iter()
        .filter(|n| ranges.iter().any(|(min, max)| *n >= *min && *n <= *max))
        .sum();

    println!("Solution for day 2 part 2: {answer}");

    Ok(())
}

fn generate_doubled_numbers_1(digits: u32) -> Vec<u64> {
    let half = digits / 2;
    let start = 10_u64.pow(half - 1);
    let end_exclusive = 10_u64.pow(half);

    (start..end_exclusive)
        .map(|n| {
            n * 10_u64.pow(half) + n
        })
        .collect()
}

fn generate_doubled_numbers_2(digits: u32) -> Vec<u64> {
    let mut results = Vec::new();

    for pattern_len in 1..=digits/2 {
        if digits % pattern_len != 0 { continue; }

        let repetitions = digits / pattern_len;
        let start = if pattern_len == 1 { 1 } else { 10_u64.pow(pattern_len - 1) };
        let end = 10_u64.pow(pattern_len);

        for pattern in start..end {
            let mut n = 0_u64;
            for _ in 0..repetitions {
                n = n * 10_u64.pow(pattern_len) + pattern;
            }
            results.push(n);
        }
    }

    results
}
