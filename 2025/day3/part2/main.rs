use std::fs;


fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to open file");
    let banks = contents
        .split('\n')
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let mut total: u64 = 0;

    for batteries in banks {
        let len = batteries.len();
        let volts = batteries.chars().collect::<Vec<char>>();
        let mut values = [0; 12];

        let mut min = 0;
        let mut max = len-11;
        for i in 0..12 {
            let mut value = min;
            for j in min..max {
                if volts[j] > volts[value] {
                    value = j;
                }
            }
            values[i] = value;
            max += 1;
            min = value+1;
        }

        let mut sum: u64 = 0;
        for i in 0..12 {
            sum = sum*10 + volts[values[i]].to_digit(10).unwrap() as u64;
        }
        total += sum;
    }

    println!("Answer: {}", total);
}

