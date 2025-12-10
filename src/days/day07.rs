use std::fs;

pub fn part1() -> std::io::Result<()> {
    let content = fs::read_to_string("data/day07.txt")?;
    let mut grid: Vec<Vec<char>> = content.lines().map(|l| l.chars().collect()).collect();
    let mut splits: u64 = 0;
    for row_n in 1..grid.len() {
        for col_n in 0..grid[row_n].len() {
            let curr_char = grid[row_n][col_n];
            let above_char = grid[row_n-1][col_n];
            if above_char == 'S' {
                grid[row_n][col_n] = '|';
            } else if above_char == '|' {
                if curr_char == '^' { // split
                    grid[row_n][col_n-1] = '|';
                    grid[row_n][col_n+1] = '|';
                    splits += 1;
                } else if curr_char == '.' {
                    grid[row_n][col_n] = '|';
                }
            }
        }
    }

    println!("Solution for day 7 part 1: {splits}");
    Ok(())
}

pub fn part2() -> std::io::Result<()> {
    let content = fs::read_to_string("data/day07_control.txt")?;
    for (i, line) in content.lines().enumerate() {
    }

    println!("Solution for day 7 part 2: ");
    Ok(())
}