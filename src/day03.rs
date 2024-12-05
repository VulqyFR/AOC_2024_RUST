use std::fs::File;
use std::io::{self, BufRead};

pub fn read_file(filename: &str, part: u32) -> Result<(), io::Error> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut sum = 0;
    let mut enabled = true;

    if part == 1 {
        for line in reader.lines() {
            sum += process_line_part1(&line?);
        }
    } else {
        for line in reader.lines() {
            let (line_sum, new_enabled) = process_line_part2(&line?, enabled);
            sum += line_sum;
            enabled = new_enabled;
        }
    }

    println!("Part {}: {}", part, sum);
    Ok(())
}

fn parse_number(chars: &[char], start: usize) -> Option<(i64, usize)> {
    let mut end = start;
    while end < chars.len() && chars[end].is_ascii_digit() {
        end += 1;
    }
    if end > start && end - start <= 3 {
        let num_str: String = chars[start..end].iter().collect();
        if let Ok(num) = num_str.parse::<i64>() {
            return Some((num, end));
        }
    }
    None
}

fn process_line_part1(line: &str) -> i64 {
    let chars: Vec<char> = line.chars().collect();
    let mut sum = 0;
    let mut i = 0;

    while i < chars.len() - 4 {
        if chars[i..].starts_with(&['m', 'u', 'l', '(']) {
            i += 4;
            if let Some((x, next_pos)) = parse_number(&chars, i) {
                i = next_pos;
                if i < chars.len() && chars[i] == ',' {
                    i += 1;
                    if let Some((y, next_pos)) = parse_number(&chars, i) {
                        i = next_pos;
                        if i < chars.len() && chars[i] == ')' {
                            sum += x * y;
                        }
                    }
                }
            }
        } else {
            i += 1;
        }
    }
    sum
}

fn process_line_part2(line: &str, mut enabled: bool) -> (i64, bool) {
    let chars: Vec<char> = line.chars().collect();
    let mut sum = 0;
    let mut i = 0;

    while i < chars.len() {
        if i + 3 < chars.len() && chars[i..].starts_with(&['d', 'o', '(', ')']) {
            enabled = true;
            i += 4;
        } else if i + 6 < chars.len()
            && chars[i..].starts_with(&['d', 'o', 'n', '\'', 't', '(', ')'])
        {
            enabled = false;
            i += 7;
        } else if enabled && i + 3 < chars.len() && chars[i..].starts_with(&['m', 'u', 'l', '(']) {
            i += 4;
            if let Some((x, next_pos)) = parse_number(&chars, i) {
                i = next_pos;
                if i < chars.len() && chars[i] == ',' {
                    i += 1;
                    if let Some((y, next_pos)) = parse_number(&chars, i) {
                        i = next_pos;
                        if i < chars.len() && chars[i] == ')' {
                            sum += x * y;
                            i += 1;
                            continue;
                        }
                    }
                }
            }
            i += 1;
        } else {
            i += 1;
        }
    }
    (sum, enabled)
}

pub fn first_star() -> Result<(), std::io::Error> {
    read_file("src/inputs/day03.txt", 1)?;
    Ok(())
}

pub fn second_star() -> Result<(), std::io::Error> {
    read_file("src/inputs/day03.txt", 2)?;
    Ok(())
}
