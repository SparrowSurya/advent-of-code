use std::{collections::{HashMap, HashSet}, fs};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point (usize, usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pair {
    m1: Point,
    m2: Point,
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to open file");
    let junctions = contents
        .split('\n')
        .map(|line| {
            line
                .to_string()
                .split(",")
                .map(|s| s.to_string().parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .map(|v| Point(v[0], v[1], v[2]))
        .collect::<Vec<Point>>();

    let mut distance: HashMap<usize, Vec<Pair>> = HashMap::new();

    for i in 0..(junctions.len()-1) {
        for j in (i+1)..junctions.len() {
            let m1 = &junctions[i];
            let m2 = &junctions[j];

            let Point(x1, y1, z1) = m1;
            let Point(x2, y2, z2) = m2;

            let dx: isize = *x1 as isize - *x2 as isize;
            let dy: isize = *y1 as isize - *y2 as isize;
            let dz: isize = *z1 as isize - *z2 as isize;
            let dist_squared = (dx*dx + dy*dy + dz*dz) as usize;

            let pair = Pair{
                m1: m1.clone(),
                m2: m2.clone(),
            };
            distance
                .entry(dist_squared)
                .or_insert_with(Vec::new)
                .push(pair.clone());
        }
    }

    let mut connections: HashMap<Point, Vec<Point>> = HashMap::new();
    let mut keys = distance.keys().cloned().collect::<Vec<usize>>();
    keys.sort();

    let mut k = 1000;

    for key in &keys {
        for pair in &distance[&key] {
            connections
                .entry(pair.m1)
                .or_insert_with(Vec::new)
                .push(pair.m2);
            connections
                .entry(pair.m2)
                .or_insert_with(Vec::new)
                .push(pair.m1);
            k -= 1;
            if k == 0 {
                break;
            }
        }
        if k == 0 {
            break;
        }
    }

    let mut visited: HashSet<Point> = HashSet::new();
    let mut circuits: Vec<Vec<Point>> = Vec::new();

    for &start in connections.keys() {
        if visited.contains(&start) {
            continue;
        }

        let mut stack = vec![start];
        let mut component = Vec::new();

        while let Some(p) = stack.pop() {
            if !visited.insert(p) {
                continue;
            }

            component.push(p);

            if let Some(neighbors) = connections.get(&p) {
                for &n in neighbors {
                    if !visited.contains(&n) {
                        stack.push(n);
                    }
                }
            }
        }

        circuits.push(component);
    }

    circuits.sort_by(|a, b| b.len().cmp(&a.len()));

    let mut total = 1;
    for i in 0..3 {
        if i >= circuits.len() {
            break;
        }
        total *= circuits[i].len();
    }

    println!("Answer: {}", total);
}
