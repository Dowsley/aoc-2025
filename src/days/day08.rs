use std::fs;

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

struct UnionFind {
    parents: Vec<usize>,
    sizes: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parents: (0..n).collect(),
            sizes: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parents[x] != x {
            self.parents[x] = self.find(self.parents[x]);
        }
        self.parents[x]
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let mut a = self.find(a);
        let mut b = self.find(b);

        if a == b {
            return false;
        }

        if self.sizes[a] < self.sizes[b] {
            std::mem::swap(&mut a, &mut b);
        }
        self.parents[b] = a;
        self.sizes[a] += self.sizes[b];

        true
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
    let mut dsu = UnionFind::new(boxes.len());
    for &(_, a, b) in pairs.iter().take(1000) {
        dsu.union(a, b);
    }

    // 2. get most frequent
    let mut circuit_sizes: Vec<usize> = (0..boxes.len())
        .filter(|&i| dsu.parents[i] == i)
        .map(|i| dsu.sizes[i])
        .collect();

    circuit_sizes.sort();
    println!("Solution for day 8 part 1: {}", circuit_sizes.iter().rev().take(3).product::<usize>());
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
    let mut dsu = UnionFind::new(boxes.len());
    let mut remaining = boxes.len();
    for &(_, a, b) in pairs.iter() {
        if dsu.union(a, b) {
            remaining -= 1;
        }

        if remaining == 1 {
            let result = boxes[a].x * boxes[b].x;
            println!("Solution for day 8 part 2: {}", result);
            break;
        }
    }

    Ok(())

}
