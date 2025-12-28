use std::fs;


fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to open file");
    let contents = contents.trim_end_matches("\n").to_string();
    let lines = contents
        .split('\n')
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();


    let rows = lines.len();
    let mut col: usize = 0;
    let mut total: usize = 0;

    loop {
        let mut nums: Vec<usize> = Vec::new();
        let op = lines[rows-1][col];
        let mut num = String::new();

        loop {
            for row in 0..rows-1 {
                let ch = lines[row][col];
                if ch.is_ascii_digit() {
                    num.push(ch);
                }
            }

            col += 1;
            if num.is_empty() {
                break;
            } else {
                let n = num.parse::<usize>().unwrap();
                nums.push(n);
                num.clear();
            }

            if col >= lines[0].len() {
                break;
            }
        }

        total += match op {
            '*' => nums.iter().product::<usize>(),
            '+' => nums.iter().sum::<usize>(),
            _ => 0,
        };

        if col >= lines[0].len() {
            break;
        }
    }

    println!("Answer: {}", total);
}
