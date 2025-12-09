use std::fs;

fn main() {
    day_1_part_1().expect("Error on day 1 part 1");
    day_1_part_2().expect("Error on day 1 part 2");
    day_2_part_1().expect("Error on day 2 part 1");
    day_2_part_2().expect("Error on day 2 part 2");
}

fn day_1_part_1() -> std::io::Result<()> {
    println!("Solution for day 1 (part 1):");

    let content = fs::read_to_string("data/day_1.txt")?;
    let mut curr_dial = 50;
    let mut pwd = 0;
    println!("The dial starts by pointing at 50.");
    for line in content.lines() {
        let dir = line.trim().chars().next().expect("Empty line");
        let mut amnt: i32 = line[1..].parse().expect("Couldn't process amount");
        amnt *= if dir == 'R' { 1 } else { -1 };

        curr_dial = ((curr_dial+amnt) % 100 + 100) % 100;

        println!("The dial is rotated {line} to point at {curr_dial}.");
        if curr_dial == 0 {
            pwd += 1;
        }
    }

    println!("The password is: {pwd}");
    Ok(())
}

fn day_1_part_2() -> std::io::Result<()> {
    println!("Solution for day 1 part 2:");

    let content = fs::read_to_string("data/day_1.txt")?;
    let mut curr_dial = 50;
    let mut pwd = 0;
    println!("The dial starts by pointing at 50.");
    for line in content.lines() {
        let dir = line.trim().chars().next().expect("Empty line");
        let mut amnt: i32 = line[1..].parse().expect("Couldn't process amount");
        amnt *= if dir == 'R' { 1 } else { -1 };
        {
            let times_passed_zero = if amnt >= 0 {
                (curr_dial + amnt) / 100
            } else {
                let steps = amnt.abs();
                if curr_dial == 0 {
                    steps / 100
                } else if steps >= curr_dial {
                    (steps - curr_dial) / 100 + 1
                } else {
                    0
                }
            };
            pwd += times_passed_zero;
        }
        curr_dial = ((curr_dial+amnt) % 100 + 100) % 100;

        println!("The dial is rotated {line} to point at {curr_dial}.");
    }

    println!("The password is: {pwd}");
    Ok(())
}

fn day_2_part_1() -> std::io::Result<()> {
    println!("Solution for day 2 part 1:");

    let mut ranges: Vec<(u64, u64)> = Vec::new();
    let content = fs::read_to_string("data/day_2.txt")?;
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

    println!("{}", answer);

    Ok(())
}

fn day_2_part_2() -> std::io::Result<()> {
    println!("Solution for day 2 part 2:");

    let mut ranges: Vec<(u64, u64)> = Vec::new();
    let content = fs::read_to_string("data/day_2.txt")?;
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

    println!("{}", answer);

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