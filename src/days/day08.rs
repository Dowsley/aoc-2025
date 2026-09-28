use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::min;

#[derive(Debug)]
struct Vector3 {
    x: i64,
    y: i64,
    z: i64
}

impl Vector3 {
    fn from_line(line: &str) -> Self {
        let parts: Vec<i64> = line.split(',')
            .map(|s| s.parse().unwrap())
            .collect();

        assert_eq!(parts.len(), 3, "Line must have exactly 3 comma-separated values");
        Vector3 { x: parts[0], y: parts[1], z: parts[2] }
    }
}



pub fn part1() -> std::io::Result<()> {
    let content = fs::read_to_string("data/day08.txt")?;
    let boxes: Vec<Vector3> = content.lines()
        .filter(|line| !line.trim().is_empty())
        .map(Vector3::from_line)
        .collect();

    // 0. assemble pairs sorted by dist
    let mut pairs: Vec<(i64, usize, usize)> = Vec::new(); // squared_dist, i, j
    for i in 0..boxes.len() {
        for j in (i+1)..boxes.len() {
            let a = &boxes[i];
            let b = &boxes[j];
            let dx = a.x - b.x;
            let dy = a.y - b.y;
            let dz = a.z - b.z;
            let squared_dist = dx*dx + dy*dy + dz*dz;
            pairs.push((squared_dist, i, j));
        }
    }
    pairs.sort_by_key(|pair| pair.0);

    // 1. assemble circuits
    let mut circuits: Vec<HashSet<usize>> = Vec::new();
    let mut box_to_circuit: HashMap<usize, usize> = HashMap::new(); // box idx to circuit hashset idx
    for i in 0..min(1000, pairs.len()) {
        let (_, a, b) = pairs[i];
        let circuit_a = box_to_circuit.get(&a).copied();
        let circuit_b = box_to_circuit.get(&b).copied();
        if circuit_a == Option::None && circuit_b == Option::None {
            circuits.push(HashSet::new());
            let idx = circuits.len()-1;
            circuits[idx].insert(a);
            circuits[idx].insert(b);
            box_to_circuit.insert(a, idx);
            box_to_circuit.insert(b, idx);
        } else {
            if circuit_a == Option::None {
                // insert into circuit_b
                let idx = circuit_b.unwrap();
                circuits[idx].insert(a);
                box_to_circuit.insert(a, idx);
            } else if circuit_b == Option::None {
                // insert into circuit_a
                let idx = circuit_a.unwrap();
                circuits[idx].insert(b);
                box_to_circuit.insert(b, idx);
            } else {
                // join both circuits (by convention, all goes to circuit_a)
                let idx_a = circuit_a.unwrap();
                let idx_b = circuit_b.unwrap();
                if idx_a != idx_b {
                    let set_b = std::mem::take(&mut circuits[idx_b]);
                    for &box_idx in &set_b {
                        box_to_circuit.insert(box_idx, idx_a);
                    }
                    circuits[idx_a].extend(set_b);
                }
            }
        }
    }

    // 2. get most frequent
    circuits.sort_by_key(|set| set.len());
    let slice_num = min(3, circuits.len());
    let mut result = 1;
    for i in circuits.len()-slice_num..circuits.len() {
        result *= &circuits[i].len();
    }

    println!("Solution for day 8 part 1: {}", result);
    Ok(())
}

pub fn part2() -> std::io::Result<()> {
    let content = fs::read_to_string("data/day08.txt")?;
    let boxes: Vec<Vector3> = content.lines()
        .filter(|line| !line.trim().is_empty())
        .map(Vector3::from_line)
        .collect();

    // 0. assemble pairs sorted by dist
    let mut pairs: Vec<(i64, usize, usize)> = Vec::new(); // squared_dist, i, j
    for i in 0..boxes.len() {
        for j in (i+1)..boxes.len() {
            let a = &boxes[i];
            let b = &boxes[j];
            let dx = a.x - b.x;
            let dy = a.y - b.y;
            let dz = a.z - b.z;
            let squared_dist = dx*dx + dy*dy + dz*dz;
            pairs.push((squared_dist, i, j));
        }
    }
    pairs.sort_by_key(|pair| pair.0);

    // 1. assemble circuits
    let mut circuits: Vec<HashSet<usize>> = Vec::new();
    let mut box_to_circuit: HashMap<usize, usize> = HashMap::new(); // box idx to circuit hashset idx
    for i in 0.. {
        let (_, a, b) = pairs[i];
        let circuit_a = box_to_circuit.get(&a).copied();
        let circuit_b = box_to_circuit.get(&b).copied();
        if circuit_a == Option::None && circuit_b == Option::None {
            circuits.push(HashSet::new());
            let idx = circuits.len()-1;
            circuits[idx].insert(a);
            circuits[idx].insert(b);
            box_to_circuit.insert(a, idx);
            box_to_circuit.insert(b, idx);
        } else {
            if circuit_a == Option::None {
                // insert into circuit_b
                let idx = circuit_b.unwrap();
                circuits[idx].insert(a);
                box_to_circuit.insert(a, idx);
            } else if circuit_b == Option::None {
                // insert into circuit_a
                let idx = circuit_a.unwrap();
                circuits[idx].insert(b);
                box_to_circuit.insert(b, idx);
            } else {
                // join both circuits (by convention, all goes to circuit_a)
                let idx_a = circuit_a.unwrap();
                let idx_b = circuit_b.unwrap();
                if idx_a != idx_b {
                    let set_b = std::mem::take(&mut circuits[idx_b]);
                    for &box_idx in &set_b {
                        box_to_circuit.insert(box_idx, idx_a);
                    }
                    circuits[idx_a].extend(set_b);
                }
            }
        }
        let circuit_idx = box_to_circuit[&a];
        if circuits[circuit_idx].len() == boxes.len() {
            let result = boxes[a].x * boxes[b].x;
            println!("Solution for day 8 part 2: {}", result);
            break;
        }
    }

    Ok(())
}
