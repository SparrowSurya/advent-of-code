use std::fs;


fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to open file");
    let data = contents
        .split('\n')
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let slice_index = data.iter().position(|s| s.is_empty()).unwrap();
    let ranges = &data[..slice_index]
        .iter()
        .map(|r| r.split("-").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>())
        .collect::<Vec<Vec<usize>>>();
    let ids = &data[slice_index+1..]
        .iter()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut total = 0;
    for id in ids {
        for range in ranges {
            let start = range[0];
            let end = range[1];
            if start <= *id && *id <= end {
                total += 1;
                break;
            }
        }
    }
    println!("Answer: {:?}", total);
}
