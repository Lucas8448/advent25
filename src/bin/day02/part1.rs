use std::fs::File;
use std::io::{BufRead, BufReader};

fn is_invalid_id(n: u64) -> bool {
    let s = n.to_string();

    // Must have even number of digits
    if s.len() % 2 != 0 {
        return false;
    }

    let half = s.len() / 2;
    let (left, right) = s.split_at(half);

    left == right
}

fn main() {
    // read file
    let file = File::open("src/bin/day02/data/input.txt")
        .expect("Failed to open file");
    let reader = BufReader::new(file);

    // comma separated ranges
    let mut values: Vec<String> = Vec::new();
    for line in reader.lines() {
        let content = line.unwrap_or_default();
        for val in content.split(',') {
            values.push(val.to_string());
        }
    }

    let mut sum: u128 = 0;

    for range in values {
        let (start, end) = range
            .split_once('-')
            .expect("Invalid range format");

        let start: u64 = start.parse().expect("Invalid number");
        let end: u64 = end.parse().expect("Invalid number");

        for n in start..=end {
            if is_invalid_id(n) {
                sum += n as u128;
            }
        }
    }

    println!("Sum of invalid IDs: {}", sum);
}
