use std::{collections::HashMap, fs};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Junction {
    x: usize,
    y: usize,
    z: usize,
    c: i32,
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to open file");
    let mut juncs = contents
        .split('\n')
        .map(|line| {
            line
                .to_string()
                .split(",")
                .map(|s| s.to_string().parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .map(|v| Junction {x: v[0], y: v[1], z: v[2], c: -1})
        .collect::<Vec<Junction>>();

    let mut distance: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();

    for i in 0..juncs.len() {
        for j in (i+1)..juncs.len() {
            let m1 = &juncs[i];
            let m2 = &juncs[j];

            let dx: isize = m1.x as isize - m2.x as isize;
            let dy: isize = m1.y as isize - m2.y as isize;
            let dz: isize = m1.z as isize - m2.z as isize;
            let dist_squared = (dx*dx + dy*dy + dz*dz) as usize;

            distance
                .entry(dist_squared)
                .or_insert_with(Vec::new)
                .push((i, j));
        }
    }

    let mut dists = distance.keys().cloned().collect::<Vec<usize>>();
    dists.sort();

    let mut circuits: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut circuit: usize = 1;
    let mut ans: usize = 0;

    for i in 0..dists.len() {
        let juncs_pair = &distance[&dists[i]];

        for (i1, i2) in juncs_pair {
            let i1 = *i1;
            let i2 = *i2;

            if juncs[i1].c == -1 && juncs[i2].c == -1 {
                juncs[i1].c = circuit as i32;
                juncs[i2].c = circuit as i32;
                circuits.insert(circuit, vec![i1, i2]);
                circuit += 1;
            } else if juncs[i1].c != -1 && juncs[i2].c == -1 {
                juncs[i2].c = juncs[i1].c;
                circuits
                    .get_mut(&(juncs[i1].c as usize))
                    .unwrap()
                    .push(i2);
            } else if juncs[i1].c == -1 && juncs[i2].c != -1 {
                juncs[i1].c = juncs[i2].c;
                circuits
                    .get_mut(&(juncs[i2].c as usize))
                    .unwrap()
                    .push(i1);
            } else if juncs[i1].c != -1 && juncs[i1].c != -1 && juncs[i1].c != juncs[i2].c {
                let moved = circuits.remove(&(juncs[i2].c as usize)).unwrap();
                for i in moved {
                    juncs[i].c = juncs[i1].c;
                    circuits.get_mut(&(juncs[i1].c as usize)).unwrap().push(i);
                }
            }

            if circuits.len() == 1 && circuits.values().next().unwrap().len() == juncs.len() {
                ans = juncs[i1].x * juncs[i2].x;
                break;
            }
        }

        if ans != 0 {
            break;
        }
    }

    println!("Answer: {}", ans);
}
