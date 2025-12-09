use std::fs;

pub fn part1() -> std::io::Result<()> {
    let content = fs::read_to_string("data/day03.txt")?;
    let mut answer = 0;
    for bank in content.lines() {
        answer += find_largest_joltage_1(bank);
    }
    println!("Solution for day 3 part 1: {answer}");
    Ok(())
}

fn find_largest_joltage_1(bank: &str) -> u32 {
    // 1. Find largest n that is not the last one: O(n)
    let mut largest = 0; // safe since the smallest battery is 1
    let mut largest_i = 0;
    for (i, battery) in bank[..bank.len()-1].chars().enumerate() {
        let n = battery.to_digit(10).expect("Error converting to number");
        if n > largest {
            largest = n;
            largest_i = i;
        }
    }
    // println!("Largest: {largest} at index {largest_i}");

    // 2. Find largest n AFTER largest number's index: O(n)
    let mut largest_after = 0;
    for (_i, battery) in bank[largest_i+1..].chars().enumerate() {
        let n = battery.to_digit(10).expect("Error converting to number");
        if n > largest_after {
            largest_after = n;
        }
    }
    // println!("Largest after: {largest_after}");

    largest*10+largest_after
}

pub fn part2() -> std::io::Result<()> {
    let content = fs::read_to_string("data/day03.txt")?;
    let mut answer = 0;
    for bank in content.lines() {
        answer += find_largest_joltage_2(bank);
    }
    println!("Solution for day 3 part 2: {answer}");
    Ok(())
}

fn find_largest_joltage_2(bank: &str) -> u64 {
    let mut answer: u64 = 0;
    let mut start = 0;

    for i in 0..12 {
        let mut largest = 0;
        let mut largest_offset = 0;


        let end = bank.len() - (12 - i - 1); // only search up to the point where we can still find enough digits

        for (j, battery) in bank[start..end].chars().enumerate() {
            let n = battery.to_digit(10).expect("Error converting to number");
            if n > largest {
                largest = n;
                largest_offset = j;
            }
        }

        start = start + largest_offset + 1;
        answer = answer * 10 + largest as u64;
    }

    answer
}