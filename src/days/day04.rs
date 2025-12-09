use std::fs;

fn get_pos(matrix: &Vec<Vec<char>>, row: usize, col: usize) -> Option<char> {
    matrix.get(row).and_then(|r| r.get(col)).copied()
}

pub fn part1() -> std::io::Result<()> {
    let content = fs::read_to_string("data/day04.txt")?;
    let matrix: Vec<Vec<char>> = content
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut answer = 0;
    for row in 0..matrix.len() {
        for col in 0..matrix[row].len() {
            let ch = matrix[row][col];
            if ch == '@' && is_accessible(&matrix, row, col) {
                answer += 1;
            }
        }
    }

    println!("Solution for day 3 part 1: {answer}");
    Ok(())
}

fn is_accessible(matrix: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    println!("Checking for row {row}, col {col}");
    let mut adjacent_rolls = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            if !(i == 0 && j == 0) {
                let new_row = row as isize + i;
                let new_col = col as isize + j;
                if let Some(ch) = get_pos(&matrix, new_row as usize, new_col as usize) {
                    if ch == '@' {
                        adjacent_rolls += 1;
                    }
                }
            }
        }
    }
    println!("{adjacent_rolls}");
    adjacent_rolls<4
}

pub fn part2() -> std::io::Result<()> {
    let content = fs::read_to_string("data/day04_control.txt")?;
    println!("Solution for day 4 part 2: ");
    Ok(())
}
