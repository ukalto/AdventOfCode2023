use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn extract_numeric_value(s: &str) -> Option<i32> {
    s.split_whitespace()
        .filter_map(|word| word.parse().ok())
        .next()
}

fn calc_first(line: &str) -> bool {
    if let Some(index) = line.find(":") {
        let game = &line[index + 2..];       
        let sets: Vec<&str> = game.split(';').collect(); 

        for set in sets {
            let mut red_count = 0;
            let mut green_count = 0;
            let mut blue_count = 0;
            let items: Vec<&str> = set.split(',').map(|item| item.trim()).collect();
            for item in items {
                if item.contains("red") {
                    if let Some(value) = extract_numeric_value(item) {
                        red_count += value;
                    }
                } else if item.contains("green") {
                    if let Some(value) = extract_numeric_value(item) {
                        green_count += value;
                    }
                } else if item.contains("blue") {
                    if let Some(value) = extract_numeric_value(item) {
                        blue_count += value;
                    }
                }
            }
            if red_count > 12 || green_count > 13 || blue_count > 14 {
                return false;
            }
        }
        true
    } else {
        println!("Pattern not found in the string.");
        false
    }
}

fn calc_second(line: &str) -> i32 {
    let mut max_red_count = 0;
    let mut max_green_count = 0;
    let mut max_blue_count = 0;

    if let Some(index) = line.find(":") {
        let game = &line[index + 2..];       
        let sets: Vec<&str> = game.split(';').collect(); 
        
        for set in sets {
            let mut red_count = 0;
            let mut green_count = 0;
            let mut blue_count = 0;
            let items: Vec<&str> = set.split(',').map(|item| item.trim()).collect();
            for item in items {
                if item.contains("red") {
                    if let Some(value) = extract_numeric_value(item) {
                        if red_count < value {
                            red_count = value;
                        }
                    }
                } else if item.contains("green") {
                    if let Some(value) = extract_numeric_value(item) {
                        if green_count < value {
                            green_count = value;
                        }
                    }
                } else if item.contains("blue") {
                    if let Some(value) = extract_numeric_value(item) {
                        if blue_count < value {
                            blue_count = value;
                        }
                    }
                }
            }
            if max_red_count < red_count {
                max_red_count = red_count;
            }
            if max_green_count < green_count {
                max_green_count = green_count;
            }
            if max_blue_count < blue_count {
                max_blue_count = blue_count;
            }
        }
    } else {
        println!("Pattern not found in the string.");
    }
    max_red_count * max_green_count * max_blue_count
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new("../day02.txt");

    let file = File::open(&path)?;
    let mut part1sum = 0;
    let mut part2sum = 0;

    let reader = BufReader::new(file);
    for (idx, line) in reader.lines().enumerate() {
        if let Ok(line) = line {
            if calc_first(&line) {
                part1sum = part1sum + 1 + idx;
            }
            part2sum += calc_second(&line);
         } else {
            eprintln!("Error reading line");
        }
    }
    println!("Part 1: {}", part1sum);
    println!("Part 2: {}", part2sum);
    Ok(())
}
