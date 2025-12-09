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
    let content = fs::read_to_string("data/day06.txt")?;
    let lines: Vec<&str> = content.lines().collect();

    let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    // Build 2D grid, padding shorter lines with spaces
    let grid: Vec<Vec<char>> = lines
        .iter()
        .map(|line| {
            let mut chars: Vec<char> = line.chars().collect();
            chars.resize(max_len, ' ');
            chars
        })
        .collect();

    let num_rows = grid.len();
    let mut grand_total: i64 = 0;
    let mut col = 0;

    while col < max_len {
        if grid.iter().all(|row| row[col] == ' ') {
            col += 1;
            continue;
        }

        let mut numbers: Vec<i64> = Vec::new();
        let mut op = '+';
        while col < max_len && !grid.iter().all(|row| row[col] == ' ') {
            let last_char = grid[num_rows - 1][col];
            if last_char == '+' || last_char == '*' {
                op = last_char;
            }
            let num_str: String = (0..num_rows - 1)
                .filter_map(|r| {
                    let ch = grid[r][col];
                    if ch.is_ascii_digit() { Some(ch) } else { None }
                })
                .collect();
            if !num_str.is_empty() {
                numbers.push(num_str.parse().unwrap());
            }
            col += 1;
        }

        let result: i64 = if op == '*' {
            numbers.iter().product()
        } else {
            numbers.iter().sum()
        };

        grand_total += result;
    }

    println!("Solution for day 6 part 2: {grand_total}");
    Ok(())
}