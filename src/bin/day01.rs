use std::fs::File;
use std::io::{self, BufReader, prelude::*};
use std::collections::HashMap;

/// Reads the input file and returns two vectors of integers parsed from each line.
fn read_input(file_path: &str) -> io::Result<(Vec<i32>, Vec<i32>)> {
    let file = File::open(file_path)?;
    let buf_reader = BufReader::new(file);
    let mut first_parts: Vec<i32> = Vec::new();
    let mut second_parts: Vec<i32> = Vec::new();

    for line in buf_reader.lines() {
        let line = line?;
        let mut parts = line.splitn(2, ' ');
        if let (Some(first_part), Some(second_part)) = (parts.next(), parts.next()) {
            match (first_part.trim().parse::<i32>(), second_part.trim().parse::<i32>()) {
                (Ok(first), Ok(second)) => {
                    first_parts.push(first);
                    second_parts.push(second);
                }
                (Err(e), _) | (_, Err(e)) => {
                    eprintln!("Error parsing line '{}': {}", line, e);
                }
            }
        }
    }

    Ok((first_parts, second_parts))
}

/// Calculates the sum of absolute differences between pairs of integers from each line.
pub fn first_star() -> io::Result<()> {
    let (mut first_parts, mut second_parts) = read_input("src/inputs/day01.txt")?;
    
    first_parts.sort();
    second_parts.sort();

    let mut sum = 0;
    for (first, second) in first_parts.iter().zip(second_parts.iter()) {
        sum += (first - second).abs();
    }

    println!("Sum of absolute differences: {}", sum);
    Ok(())
}

/// Calculates a similarity score based on the occurrences of integers in the second parts of each line.
pub fn second_star() -> io::Result<()> {
    let (first_parts, second_parts) = read_input("src/inputs/day01.txt")?;

    let mut occurrences = HashMap::new();
    for &num in &second_parts {
        *occurrences.entry(num).or_insert(0) += 1;
    }

    let mut similarity_score = 0;
    for &num in &first_parts {
        if let Some(&count) = occurrences.get(&num) {
            similarity_score += num * count;
        }
    }

    println!("Total similarity score: {}", similarity_score);
    Ok(())
}

fn main() -> std::io::Result<()> {
    println!("Advent of Code 2024 - Day 1 - https://adventofcode.com/2024/day/1 \n\n");
    println!("First star: \n");
    let start = std::time::Instant::now();
    first_star()?;
    println!("Elapsed time: {:?}\n\n", start.elapsed());
    println!("Second star:\n");
    let start = std::time::Instant::now();
    second_star()?;    
    println!("Elapsed time: {:?}\n", start.elapsed());
    Ok(())
}