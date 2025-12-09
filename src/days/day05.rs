use std::fs;

pub fn part1() -> std::io::Result<()> {
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    let mut ids: Vec<u64> = Vec::new();

    let mut read_range = true;
    let content = fs::read_to_string("data/day05.txt")?;
    for line in content.lines() {
        if line.is_empty() {
            read_range = false;
            continue;
        }

        if read_range {
            let (start, end) = line.split_once('-').unwrap();
            ranges.push((start.parse().unwrap(), end.parse().unwrap()));
        } else {
            ids.push(line.parse().unwrap());
        }
    }

    let mut fresh_count = 0;
    for id in ids {
        for (start, end) in &ranges {
            if id >= *start && id <= *end {
                fresh_count += 1;
                break;
            }
        }

    }

    println!("Solution for day 5 part 1: {fresh_count}");
    Ok(())
}

pub fn part2() -> std::io::Result<()> {
    let content = fs::read_to_string("data/day05_control.txt")?;
    println!("Solution for day 5 part 2: ");
    Ok(())
}