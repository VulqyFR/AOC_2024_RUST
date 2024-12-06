use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file() -> Result<BufReader<File>, std::io::Error> {
    let file = File::open("src/inputs/day03.txt")?;
    Ok(BufReader::new(file))
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

pub fn first_star() -> Result<(), std::io::Error> {
    let reader = read_file()?;
    let mut total = 0;

    for line in reader.lines() {
        let chars: Vec<char> = line?.chars().collect();
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
                                total += x * y;
                            }
                        }
                    }
                }
            }
            i += 1;
        }
    }

    println!("First star: {}", total);
    Ok(())
}

pub fn second_star() -> Result<(), std::io::Error> {
    let reader = read_file()?;
    let mut total = 0;
    let mut enabled = true;

    for line in reader.lines() {
        let chars: Vec<char> = line?.chars().collect();
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
            } else if enabled
                && i + 3 < chars.len()
                && chars[i..].starts_with(&['m', 'u', 'l', '('])
            {
                i += 4;
                if let Some((x, next_pos)) = parse_number(&chars, i) {
                    i = next_pos;
                    if i < chars.len() && chars[i] == ',' {
                        i += 1;
                        if let Some((y, next_pos)) = parse_number(&chars, i) {
                            i = next_pos;
                            if i < chars.len() && chars[i] == ')' {
                                total += x * y;
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
    }

    println!("Second star: {}", total);
    Ok(())
}
