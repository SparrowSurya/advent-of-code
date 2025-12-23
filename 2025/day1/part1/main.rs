use std::fs;
use std::io::{self, BufRead};


fn main() {
    let file = fs::File::open("input.txt").expect("Failed to open file");
    let file_reader = io::BufReader::new(file);

    let mut zeros = 0;
    let mut dial = 50;

    for line in file_reader.lines() {
        let line = line.unwrap();

        let mut chars = line.chars();
        let rotation: char = chars.next().unwrap();
        let value: i64 = chars.collect::<String>().parse().unwrap();

        match rotation {
            'R' => dial = (dial + value) % 100,
            'L' => dial = (dial - value) % 100,
            _ => {},
        };
        if dial < 0 {
            dial = 100 + dial;
        }

        if dial == 0 {
            zeros += 1;
        }
    }

    println!("Answer: {}", zeros);
}

