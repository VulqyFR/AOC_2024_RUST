use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn safe_slice(line: &str, start: usize, length: usize) -> Option<String> {
    if start + length <= line.len() {
        Some(line[start..start + length].to_string())
    } else {
        None
    }
}

pub fn first_star() -> Result<(), std::io::Error> {
    let file = File::open("src/inputs/day04.txt")?;
    let buf_reader = BufReader::new(file);
    let mut sum = 0;
    let mut grid = Vec::new();

    for line in buf_reader.lines() {
        let line = line?;
        grid.push(line);
    }

    let rows = grid.len();
    let cols = grid[0].len();

    // Horizontal
    for line in &grid {
        for (index, _) in line.chars().enumerate() {
            if let Some(slice) = safe_slice(line, index, 4) {
                if slice.contains("XMAS") || slice.contains("SAMX") {
                    sum += 1;
                }
            }
        }
    }

    // Vertical
    for col in 0..cols {
        let mut vertical = String::new();
        for row in 0..rows {
            vertical.push(grid[row].chars().nth(col).unwrap());
        }
        for index in 0..vertical.len() - 3 {
            if let Some(slice) = safe_slice(&vertical, index, 4) {
                if slice.contains("XMAS") || slice.contains("SAMX") {
                    sum += 1;
                }
            }
        }
    }

    // Diagonal top left to bottom right
    for row in 0..rows - 3 {
        for col in 0..cols - 3 {
            let mut diagonal = String::new();
            for i in 0..4 {
                diagonal.push(grid[row + i].chars().nth(col + i).unwrap());
            }
            if diagonal.contains("XMAS") || diagonal.contains("SAMX") {
                sum += 1;
            }
        }
    }

    // Diagonal bottom right to top left
    for row in 0..rows - 3 {
        for col in (3..cols).rev() {
            let mut diagonal = String::new();
            for i in 0..4 {
                diagonal.push(grid[row + i].chars().nth(col - i).unwrap());
            }
            if diagonal.contains("XMAS") || diagonal.contains("SAMX") {
                sum += 1;
            }
        }
    }

    println!("Sum: {}", sum);
    Ok(())
}

pub fn second_star() -> Result<(), std::io::Error> {
    Ok(())
}
