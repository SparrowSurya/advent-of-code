use std::fs;


fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to open file");
    let ranges = contents
        .split(',')
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let mut total = 0;

    for range in ranges {
        let end_points = range.split("-").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        let start = *(end_points.iter().nth(0).unwrap());
        let end = *(end_points.iter().nth(1).unwrap());

        for value in start..=end {
            let val = value.to_string();
            let mut invalid = false;

            for i in 1..=(val.len()/2) {
                if val.len() % i != 0 {
                    continue;
                }

                let substr = val[..i].to_string();
                let repeated = substr.repeat(val.len()/i);
                if repeated == val {
                    invalid = true;
                    break;
                }
            }

            if invalid {
                total += value;
            }
        }
    }

    println!("Answer: {}", total);
}

