use std::fs;


fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to open file");
    let data = contents
        .split('\n')
        .map(|line| {
            line.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

    let cols = data[0].len();
    let rows = data.len();
    let mut total: usize = 0;

    for c in 0..cols {
        let op = &data[rows-1][c];
        let mut col_total: usize = if op == "*" {1} else {0};

        for r in 0..rows-1 {
            let num = &data[r][c].parse::<usize>().unwrap();
            if op == "*" {
                col_total *= num;
            } else if op == "+" {
                col_total += num;
            }
        }

        total += col_total;
    }

    println!("Answer: {:?}", total);
}
