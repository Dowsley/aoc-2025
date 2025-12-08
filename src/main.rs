use std::fs;

fn main() {
    // day_1_part_1().expect("Error on day 1 part 1");
    day_1_part_2().expect("Error on day 1 part 2");
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