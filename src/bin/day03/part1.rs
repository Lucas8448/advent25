use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("src/bin/day03/data/input.txt").expect("Failed to open file");
    let reader = BufReader::new(file);
    
    let mut total_joltage = 0; // Total Joltage
    for line in reader.lines() {
        // read line, and put each number in the line into a vector
        let content = line.unwrap_or("".to_string());
        let mut batteries: Vec<u32> = Vec::new();
        for val in content.chars() {
            let battery: u32 = val.to_digit(10).unwrap_or(0);
            batteries.push(battery);
        }
        
        // Find max joltage by trying each position as the tens digit
        // and finding the best ones digit to the right of it
        let mut max_joltage = 0;
        for i in 0..batteries.len() - 1 {
            let max_right = batteries[i + 1..].iter().max().unwrap_or(&0);
            let joltage = batteries[i] * 10 + max_right;
            if joltage > max_joltage {
                max_joltage = joltage;
            }
        }

        total_joltage += max_joltage;
    }

    println!("Total Joltage: {}", total_joltage); 
}