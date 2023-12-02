use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn calc_first(line: &str) -> i32 {
    let first_digit = line.chars().find(|c| c.is_digit(10)).unwrap_or('0').to_digit(10).unwrap_or(0);
    let last_digit = line.chars().rev().find(|c| c.is_digit(10)).unwrap_or('0').to_digit(10).unwrap_or(0);

    first_digit as i32 * 10 + last_digit as i32
}


fn calc_second(line: &str, digit_map: &HashMap<&str, i32>) -> i32 {
    let mut first_number = 0;
    let mut last_number = 0;
    let mut first_number_found = false;

    let mut current_number = String::new();

    for (i, c) in line.chars().enumerate() {
        for cc in line.chars().skip(i) {
            current_number.push(cc);
            if let Some(digit) = digit_map.get(&current_number.as_str()) {
                if !first_number_found {
                    first_number_found = true;
                    first_number = *digit;
                }
                last_number = *digit;
            }
        }
        current_number.clear();

        if let Some(digit) = c.to_digit(10) {
            if !first_number_found {
                first_number_found = true;
                first_number = digit as i32;
            }
            last_number = digit as i32;
        } 
    }
    first_number*10 + last_number
}

fn main() -> Result<(), Box<dyn Error>> {
    // Create a path to the desired file
    let path = Path::new("../day01.txt");

    // Define a mapping of spelled-out digits to their numeric values
    let digit_map: HashMap<&str, i32> = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
    .iter()
    .cloned()
    .collect();

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = File::open(&path)?;
    let mut part1sum = 0;
    let mut part2sum = 0;

    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Ok(line) = line {
            part1sum += calc_first(&line);
            part2sum += calc_second(&line, &digit_map);
        } else {
            eprintln!("Error reading line");
        }
    }
    println!("Part 1: {}", part1sum);
    println!("Part 2: {}", part2sum);

    Ok(())
}



