use std::fs;


fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to open file");
    let data = contents
        .split('\n')
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let slice_index = data.iter().position(|s| s.is_empty()).unwrap();
    let mut ranges = data[..slice_index]
        .iter()
        .map(|r| r.split("-").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>())
        .collect::<Vec<Vec<usize>>>();
    ranges.sort_by(|a, b| a[0].cmp(&b[0]));

    let mut total = 0;
    let mut last = if ranges[0][0] == 0 { ranges[0][0] } else { ranges[0][0]-1 };

    for range in ranges {
        let start = range[0];
        let end = range[1];

        if start > last {
            total += end - start + 1;
        } else if start < last {
            if end > last {
                total += end - last;
            }
        } else {
            total += end - start;
        }

        last = if end >= last { end } else { last };
    }

    println!("Answer: {}", total);
}
