use std::fs;

pub fn part1() -> std::io::Result<()> {
    let mut result_table: Vec<i64> = Vec::new();
    let mut op_table: Vec<char> = Vec::new();

    let content = fs::read_to_string("data/day06.txt")?;
    for (i, line) in content.lines().rev().enumerate() {
        for (j, part) in line.trim().split_whitespace().enumerate() {
            if i == 0 {
                let op = part.chars().next().unwrap();
                op_table.push(op);
                result_table.push(if op == '*' { 1 } else { 0 });
            } else {
                let n: i64 = part.trim().parse().unwrap();
                if op_table[j] == '*' {
                    result_table[j] *= n;
                } else {
                    result_table[j] += n;
                }
            }
        }
    }

    let mut answer: i64 = 0;
    for r in result_table {
        answer += r;
    }
    println!("Solution for day 6 part 1: {answer}");
    Ok(())
}

pub fn part2() -> std::io::Result<()> {
    let content = fs::read_to_string("data/day06_control.txt")?;
    for _line in content.lines() {

    }

    println!("Solution for day 6 part 2: ");
    Ok(())
}