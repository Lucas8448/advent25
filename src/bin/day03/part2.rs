use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("src/bin/day03/data/input.txt").expect("Failed to open file");
    let reader = BufReader::new(file);
    
    let mut total_joltage: u64 = 0; // Total Joltage
    for line in reader.lines() {
        let content = line.unwrap_or("".to_string());
        let digits: Vec<u32> = content
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect();
        
        // Find the biggest 12-digit number by selecting digits left to right
        // At each step, pick the largest digit possible while ensuring
        // we still have enough digits remaining to complete 12 digits
        let target_len = 12;
        let mut result: Vec<u32> = Vec::new();
        let mut start_idx = 0;
        
        for i in 0..target_len {
            let digits_needed = target_len - i;
            // We can search from start_idx to (digits.len() - digits_needed) inclusive
            let end_idx = digits.len() - digits_needed + 1;
            
            // Find the maximum digit and its position in the valid range
            let mut max_digit = 0;
            let mut max_pos = start_idx;
            for j in start_idx..end_idx {
                if digits[j] > max_digit {
                    max_digit = digits[j];
                    max_pos = j;
                }
            }
            
            result.push(max_digit);
            start_idx = max_pos + 1; // Move past the digit we just used
        }
        
        // Convert the 12 digits to a number
        let number: u64 = result.iter().fold(0u64, |acc, &d| acc * 10 + d as u64);
        total_joltage += number;
    }

    println!("Total Joltage: {}", total_joltage); 
}