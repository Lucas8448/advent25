use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("src/bin/day01/data/input.txt").expect("Failed to open file");
    let reader = BufReader::new(file);
    
    let mut times_hit0 = 0; // Counter for how many times we hit 0
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

        // add rollover logic for 0..99
        while current_number < 0 {
            current_number += 100;
        }
        while current_number >= 100 {
            current_number -= 100;
        }

        // add to counter if we hit 0
        if current_number == 0 {
            times_hit0 += 1;
        }
    }

    println!("Times hit 0: {}", times_hit0); 
}