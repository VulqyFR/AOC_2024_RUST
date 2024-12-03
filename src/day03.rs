use std::{
    fs::File,
    io::{BufRead, BufReader},
};

struct Multiplication {
    first: i32,
    second: i32,
}

fn is_instruction(line: &str, index: usize, instruction: &str) -> bool {
    if index + instruction.len() > line.len() {
        return false;
    }
    &line[index..index + instruction.len()] == instruction
}

fn parse_multiplication(line: &str, index: usize) -> Option<Multiplication> {
    if index + 4 > line.len() || &line[index..index + 4] != "mul(" {
        return None;
    }

    let mut num_str = String::new();
    let mut i = index + 4;
    let mut first_digit = 0;
    let mut second_digit = 0;

    while i < line.len() {
        let char = line.chars().nth(i).unwrap();
        if char == ',' {
            if let Ok(num) = num_str.parse::<i32>() {
                first_digit = num;
                num_str = String::new();
                i += 1;
                break;
            }
            return None;
        }
        if !char.is_digit(10) {
            return None;
        }
        num_str.push(char);
        i += 1;
    }

    while i < line.len() {
        let char = line.chars().nth(i).unwrap();
        if char == ')' {
            if let Ok(num) = num_str.parse::<i32>() {
                second_digit = num;
                return Some(Multiplication {
                    first: first_digit,
                    second: second_digit,
                });
            }
            return None;
        }
        if !char.is_digit(10) {
            return None;
        }
        num_str.push(char);
        i += 1;
    }
    None
}

fn read_file(path: &str, star: u8) -> Result<(), std::io::Error> {
    let file = File::open(path)?;
    let buf_reader = BufReader::new(file);
    let mut sum = 0;

    for line in buf_reader.lines() {
        let line = line?;
        if star == 1 {
            for index in 0..line.len() {
                if let Some(mult) = parse_multiplication(&line, index) {
                    sum += mult.first * mult.second;
                }
            }
        } else {
            let mut muls_enabled = true;
            let mut index = 0;

            while index < line.len() {
                if is_instruction(&line, index, "do()") {
                    muls_enabled = true;
                    index += 4;
                } else if is_instruction(&line, index, "don't()") {
                    muls_enabled = false;
                    index += 7;
                } else {
                    if muls_enabled {
                        if let Some(mult) = parse_multiplication(&line, index) {
                            sum += mult.first * mult.second;
                            while index < line.len() && line.chars().nth(index).unwrap() != ')' {
                                index += 1;
                            }
                            index += 1;
                            continue;
                        }
                    }
                    index += 1;
                }
            }
        }
    }
    println!("Sum for star {}: {}", star, sum);
    Ok(())
}

pub fn first_star() -> Result<(), std::io::Error> {
    read_file("src/inputs/day03.txt", 1)?;
    Ok(())
}

pub fn second_star() -> Result<(), std::io::Error> {
    read_file("src/inputs/day03.txt", 2)?;
    Ok(())
}
