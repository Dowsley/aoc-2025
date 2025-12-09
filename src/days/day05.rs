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
    let mut ranges: Vec<(u64, u64)> = Vec::new();

    let content = fs::read_to_string("data/day05.txt")?;
    for line in content.lines() {
        if line.is_empty() {
            break;
        }
        let (start, end) = line.split_once('-').unwrap();
        ranges.push((start.parse().unwrap(), end.parse().unwrap()));
    }

    ranges.sort_by_key(|&(start, _)| start);
    let mut merged: Vec<(u64, u64)> = Vec::new();
    if !ranges.is_empty() {
        let mut current_start = ranges[0].0;
        let mut current_end = ranges[0].1;

        for &(start, end) in &ranges[1..] {
            if start <= current_end + 1 {
                current_end = current_end.max(end);
            } else {
                merged.push((current_start, current_end));
                current_start = start;
                current_end = end;
            }
        }
        merged.push((current_start, current_end));
    }

    let total: u64 = merged.iter()
        .map(|(start, end)| end - start + 1)
        .sum();

    println!("Solution for day 5 part 2: {total}");
    Ok(())
}