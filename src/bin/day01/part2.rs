use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("src/bin/day01/data/input.txt").expect("Failed to open file");
    let reader = BufReader::new(file);
    
    let mut times_hit0_end_rotation = 0; // Counter for how many times we hit 0 at the end of rotation
    let mut times_hit0_during_rotation = 0; // Counter for how many times we hit 0 during a rotation
    let mut current_number = 50; // Starting input on safe
    for line in reader.lines() {
        // split line into direction and number
        let content = line.unwrap_or("L0".to_string());
        let direction = &content[0..1];
        let number: i32 = content[1..].parse().unwrap_or(0);

        // determine direction and update current_number
        match direction {
            "L" => current_number -= number,
            "R" => current_number += number,
            _ => (),
        }

        // Count how many times we pass through 0 (including landing on it)
        // For L (going down): we pass 0 each time we go below 0
        // For R (going up): we pass 0 each time we go above 99
        if current_number < 0 {
            // Number of times we wrapped around (passed through 0)
            // e.g., -1 means we passed 0 once, -101 means twice
            let wraps = (-current_number + 99) / 100; // ceiling division
            times_hit0_during_rotation += wraps;
            current_number = ((current_number % 100) + 100) % 100;
        } else if current_number >= 100 {
            // Number of times we wrapped around (passed through 0)
            // e.g., 100 means we passed 0 once, 200 means twice
            let wraps = current_number / 100;
            times_hit0_during_rotation += wraps;
            current_number = current_number % 100;
        }
    }

    let total_times_hit0 = times_hit0_end_rotation + times_hit0_during_rotation;
    println!("Times hit 0 at end of rotation: {}", total_times_hit0);
}