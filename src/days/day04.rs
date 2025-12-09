use std::fs;
use rayon::prelude::*;

fn get_pos(matrix: &Vec<Vec<char>>, row: usize, col: usize) -> Option<char> {
    matrix.get(row).and_then(|r| r.get(col)).copied()
}

pub fn part1() -> std::io::Result<()> {
    let content = fs::read_to_string("data/day04.txt")?;
    let matrix: Vec<Vec<char>> = content
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let answer: usize = (0..matrix.len())
        .into_par_iter()
        .map(|row| {
            (0..matrix[row].len())
                .filter(|&col| {
                    let ch = matrix[row][col];
                    ch == '@' && is_accessible(&matrix, row, col)
                })
                .count()
        })
        .sum();


    println!("Solution for day 3 part 1: {answer}");
    Ok(())
}

fn is_accessible(matrix: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
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
    adjacent_rolls<4
}

pub fn part2() -> std::io::Result<()> {
    let content = fs::read_to_string("data/day04.txt")?;
    let mut matrix: Vec<Vec<char>> = content
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut total_removed = 0;
    loop {
        // 1. find all accessible rolls in current state in PARALLEL
        let to_remove: Vec<(usize, usize)> = (0..matrix.len())
            .into_par_iter()
            .flat_map(|row| {
                (0..matrix[row].len())
                    .filter(|&col| {
                        matrix[row][col] == '@' && is_accessible(&matrix, row, col)
                    })
                    .map(move |col| (row, col))
                    .collect::<Vec<_>>()
            })
            .collect();

        if to_remove.is_empty() {
            break; // no more rolls can be removed
        }

        // 2. remove all marked rolls SEQUENTIALLY
        for (row, col) in &to_remove {
            matrix[*row][*col] = '.';
        }

        total_removed += to_remove.len();
    }

    println!("Solution for day 4 part 2: {total_removed}");
    Ok(())
}