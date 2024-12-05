use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn read_grid(filename: &str) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(filename)?;
    let buf_reader = BufReader::new(file);
    let mut grid = Vec::new();

    for line in buf_reader.lines() {
        grid.push(line?);
    }
    Ok(grid)
}

fn safe_slice(line: &str, start: usize, length: usize) -> Option<String> {
    if start + length <= line.len() {
        Some(line[start..start + length].to_string())
    } else {
        None
    }
}

fn find_xmas(grid: &[String]) -> usize {
    let mut count = 0;
    let rows = grid.len();
    let cols = grid[0].len();

    // Horizontal
    for line in grid {
        for (index, _) in line.chars().enumerate() {
            if let Some(slice) = safe_slice(line, index, 4) {
                if slice.contains("XMAS") || slice.contains("SAMX") {
                    count += 1;
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
                    count += 1;
                }
            }
        }
    }

    // Diagonal top-left to bottom-right
    for row in 0..rows - 3 {
        for col in 0..cols - 3 {
            let mut diagonal = String::new();
            for i in 0..4 {
                diagonal.push(grid[row + i].chars().nth(col + i).unwrap());
            }
            if diagonal.contains("XMAS") || diagonal.contains("SAMX") {
                count += 1;
            }
        }
    }

    // Diagonal bottom-right to top-left
    for row in 0..rows - 3 {
        for col in (3..cols).rev() {
            let mut diagonal = String::new();
            for i in 0..4 {
                diagonal.push(grid[row + i].chars().nth(col - i).unwrap());
            }
            if diagonal.contains("XMAS") || diagonal.contains("SAMX") {
                count += 1;
            }
        }
    }
    count
}

fn find_x_pattern(grid: &Vec<Vec<char>>) -> i32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for i in 1..rows - 1 {
        for j in 1..cols - 1 {
            // Check center A
            if grid[i][j] != 'A' {
                continue;
            }

            // Check diagonal patterns
            let patterns = [
                // Top-left to bottom-right
                ((i - 1, j - 1), (i + 1, j + 1)),
                // Top-right to bottom-left
                ((i - 1, j + 1), (i + 1, j - 1)),
            ];

            let mut valid_diagonals = 0;

            for ((top_i, top_j), (bottom_i, bottom_j)) in patterns {
                // Check forward MAS
                if (grid[top_i][top_j] == 'M' && grid[bottom_i][bottom_j] == 'S') ||
                   // Check backward SAM
                   (grid[top_i][top_j] == 'S' && grid[bottom_i][bottom_j] == 'M')
                {
                    valid_diagonals += 1;
                }
            }

            // If both diagonals form valid patterns
            if valid_diagonals == 2 {
                count += 1;
            }
        }
    }
    count
}

pub fn first_star() -> Result<(), std::io::Error> {
    let grid = read_grid("src/inputs/day04.txt")?;
    let count = find_xmas(&grid);
    println!("Part 1: {}", count);
    Ok(())
}

fn convert_grid(grid: &[String]) -> Vec<Vec<char>> {
    grid.iter().map(|line| line.chars().collect()).collect()
}

pub fn second_star() -> Result<(), std::io::Error> {
    let grid = read_grid("src/inputs/day04.txt")?;
    let char_grid = convert_grid(&grid);
    let count = find_x_pattern(&char_grid);
    println!("Part 2: {}", count);
    Ok(())
}
