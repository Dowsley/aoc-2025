use std::fs;

pub fn part1() -> std::io::Result<()> {
    let content = fs::read_to_string("data/day01.txt")?;
    let mut curr_dial = 50;
    let mut pwd = 0;
    for line in content.lines() {
        let dir = line.trim().chars().next().expect("Empty line");
        let mut amnt: i32 = line[1..].parse().expect("Couldn't process amount");
        amnt *= if dir == 'R' { 1 } else { -1 };

        curr_dial = ((curr_dial+amnt) % 100 + 100) % 100;

        if curr_dial == 0 {
            pwd += 1;
        }
    }

    println!("Solution for day 1 (part 1): {pwd}");
    Ok(())
}

pub fn part2() -> std::io::Result<()> {
    let content = fs::read_to_string("data/day01.txt")?;
    let mut curr_dial = 50;
    let mut pwd = 0;
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
    }

    println!("Solution for day 1 part 2: {pwd}");
    Ok(())
}
