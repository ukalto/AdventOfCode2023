use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn find_hits(line: &str) -> i32 {
    let mut count_hits = 0;
    if let Some(index) = line.find(":") {
        let cards = &line[index + 2..];
        let parts: Vec<&str> = cards.split('|').map(|s| s.trim()).collect();
        if parts.len() == 2 {
            let numbers: Vec<&str> = parts[0].split(' ').map(|s| s.trim()).collect();
            let game: Vec<&str> = parts[1].split(' ').map(|s| s.trim()).filter(|&s| !s.is_empty()).collect();

            for g in game {
                if numbers.contains(&g) {
                    count_hits += 1;
                }
            }
        } else {
            println!("Invalid format");
        }
    } else {
        println!("Pattern not found in the string.");
    }
    count_hits
}

fn calc_first(line: &str) -> i32 {
    let count_hits = find_hits(&line);
    let mut final_val = 1;
    if count_hits > 1 {
        for _ in 1..count_hits {
            final_val *= 2;
        }
    }
    final_val
}

fn calc_second(line: &str, save_before_hits: i32) -> i32 {
    let count_hits = find_hits(&line);
    count_hits * save_before_hits
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new("../day04.txt");

    let file = File::open(&path)?;
    let mut part1sum = 0;
    let mut part2sum = 0;
    let mut save_before_hits = 1;

    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Ok(line) = line {
            part1sum += calc_first(&line);
            println!("save_before_hits: {}", save_before_hits);
            part2sum += calc_second(&line, save_before_hits);
            save_before_hits = find_hits(&line);
        } else {
            eprintln!("Error reading line");
        }
    }
    println!("Part 1: {}", part1sum);
    println!("Part 2: {}", part2sum);

    Ok(())
}
