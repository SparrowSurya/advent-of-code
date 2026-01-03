use std::fs;


fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to open file");
    let grid = contents
        .split('\n')
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let start_y = grid.iter().position(|r| r.contains(&'S')).unwrap();
    let rows = grid.len();
    let cols = grid[0].len();

    let mut rays: Vec<usize> = grid[start_y + 1]
        .iter()
        .enumerate()
        .map(|(x, _)| {
            if grid[start_y][x] == 'S' { 1 } else { 0 }
        })
        .collect();

    for y in (start_y+2)..rows {
        let mut next_rays: Vec<usize> = vec![0; rays.len()];

        for x in 0..cols {
            if grid[y][x] == '^' {
                if x > 0 {
                    next_rays[x-1] += rays[x];
                }
                if x+1 < cols {
                    next_rays[x+1] += rays[x];
                }
            } else {
                next_rays[x] += rays[x];
            }
        }

        rays = next_rays;
    }

    let total = rays.iter().sum::<usize>();
    println!("Answer: {}", total);
}
