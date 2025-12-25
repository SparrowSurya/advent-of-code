use std::fs;


fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to open file");
    let banks = contents
        .split('\n')
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let mut total = 0;

    for batteries in banks {
        let len = batteries.len();
        let volts = batteries.chars().collect::<Vec<char>>();

        let mut a = 0;
        for i in 0..(len-1) {
            if volts[i] > volts[a] {
                a = i;
            }
        }

        let mut b = a+1;
        for i in (a+1)..len {
            if volts[i] > volts[b] {
                b = i;
            }
        }

        total += volts[a].to_digit(10).unwrap()*10 + volts[b].to_digit(10).unwrap();
    }

    println!("Answer: {}", total);
}

