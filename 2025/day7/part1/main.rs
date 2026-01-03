use std::fs;


fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to open file");
    let mut grid = contents
        .split('\n')
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let start_y = grid.iter().position(|r| r.contains(&'S')).unwrap();
    let start_x = grid[start_y].iter().position(|c| *c == 'S').unwrap();
    let rows = grid.len();
    let cols = grid[0].len();

    let mut stack: Vec<(usize, usize)> = vec![(start_x, start_y+1)];
    let mut total: usize = 0;

    while !stack.is_empty() {
        let (x, mut y) = stack.pop().unwrap();

        while y < rows && grid[y][x] == '.' {
            grid[y][x] = '|';
            y += 1;
        }

        if y < rows && grid[y][x] == '^' {
            total += 1;
            if x+1 < cols && grid[y][x+1] == '.' {
                stack.push((x+1, y));
            }
            if x > 0 && grid[y][x-1] == '.' {
                stack.push((x-1, y));
            }
        }
    }

    println!("Answer: {}", total);
}
