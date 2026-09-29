use std::cmp::min;
use std::cmp::max;
use std::collections::HashMap;
use std::fs;

struct Vector2 {
    x: i64,
    y: i64,
}

impl Vector2 {
    fn from_line(line: &str) -> Self {
        let parts: Vec<i64> = line.split(',')
            .map(|s| s.parse().unwrap())
            .collect();

        assert_eq!(parts.len(), 2, "Line must have exactly 2 comma-separated values");
        Vector2 { x: parts[0], y: parts[1] }
    }
}

pub fn part1() -> std::io::Result<()> {
    let content = fs::read_to_string("data/day09.txt")?;
    let points: Vec<Vector2> = content.lines()
        .filter(|line| !line.trim().is_empty())
        .map(Vector2::from_line)
        .collect();

    let mut points_by_x: HashMap<i64, (i64, i64)> = HashMap::new();
    for p in points {
        let (min_y, max_y) = points_by_x
            .entry(p.x)
            .or_insert((p.y, p.y));
        (*min_y) = min(*min_y, p.y);
        (*max_y) = max(*max_y, p.y);
    }

    let mut point_groups: Vec<(i64, (i64, i64))> = points_by_x.into_iter().collect();
    point_groups.sort_by_key(|p| p.0);

    let mut bottom_left: Vec<Vector2> = Vec::new();
    let mut top_left: Vec<Vector2> = Vec::new();
    let mut bottom_right: Vec<Vector2> = Vec::new();
    let mut top_right: Vec<Vector2> = Vec::new();

    let mut max_y = i64::MIN;
    let mut min_y = i64::MAX;
    for &(x, (local_min_y, local_max_y)) in &point_groups {
        if local_min_y < min_y {
            bottom_left.push(Vector2 { x, y: local_min_y });
            min_y = local_min_y;
        }
        if local_max_y > max_y {
            top_left.push(Vector2 { x, y: local_max_y });
            max_y = local_max_y;
        }
    }

    max_y = i64::MIN;
    min_y = i64::MAX;
    for &(x, (local_min_y, local_max_y)) in point_groups.iter().rev() {
        if local_min_y < min_y {
            bottom_right.push(Vector2 { x, y: local_min_y });
            min_y = local_min_y;
        }
        if local_max_y > max_y {
            top_right.push(Vector2 { x, y: local_max_y });
            max_y = local_max_y;
        }
    }

    let mut largest_area = 0;
    for p1 in &top_right {
        for p2 in &bottom_left {
            let area = ((p1.x - p2.x).abs() + 1) * ((p1.y - p2.y).abs() + 1);
            largest_area = max(largest_area, area); 
        }
    }

    for p1 in &top_left {
        for p2 in &bottom_right {
            let area = ((p1.x - p2.x).abs() + 1) * ((p1.y - p2.y).abs() + 1);
            largest_area = max(largest_area, area); 
        }
    }

    println!("Solution for day 9 part 1: {}", largest_area);
    Ok(())
}

pub fn part2() -> std::io::Result<()> {
    let content = fs::read_to_string("data/day09_control.txt")?;
    // let boxes: Vec<Vector3> = content.lines()
    //     .filter(|line| !line.trim().is_empty())
    //     .map(Vector3::from_line)
    //     .collect();

    println!("Solution for day 9 part 2: {}", 0);
    Ok(())
}
