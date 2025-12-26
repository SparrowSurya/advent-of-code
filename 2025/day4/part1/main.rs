use std::fs;


fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to open file");
    let grid = contents
        .split('\n')
        .map(|s| s.to_string().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let height: i32 = grid.len() as i32;
    let width: i32 = grid[0].len() as i32;

    let mut total = 0;
    let adjacents: [(i32, i32); 8] = [
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    for y in 0..height {
        for x in 0..width {
            if grid[y as usize][x as usize] != '@' {
                continue;
            }

            let mut adj_rolls = 0;
            for (dx, dy) in adjacents {
                let nx = x+dx;
                let ny = y+dy;
                let in_width = 0 <= nx && nx < width;
                let in_height = 0 <= ny && ny < height;
                if in_width && in_height && grid[ny as usize][nx as usize] == '@' {
                    adj_rolls += 1;
                }
            }

            if adj_rolls < 4 {
                total += 1;
            }
        }
    }

    println!("Answer: {}", total);
}
