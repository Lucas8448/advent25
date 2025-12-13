use std::fs::File;
use std::io::{BufRead, BufReader};

fn is_invalid_id(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();

    for unit_len in 1..=len / 2 {
        if len % unit_len != 0 {
            continue;
        }

        let times = len / unit_len;
        let unit = &s[..unit_len];

        if unit.repeat(times) == s {
            return true; // repeated at least twice
        }
    }

    false
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
