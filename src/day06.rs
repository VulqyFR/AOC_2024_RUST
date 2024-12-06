use std::collections::HashSet;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    fn rotate_clockwise(self) -> Direction {
        match self {
            Direction::N => Direction::E,
            Direction::E => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::N,
        }
    }
}

fn read_file() -> Result<BufReader<File>, std::io::Error> {
    let file = File::open("src/inputs/day06.txt")?;
    Ok(BufReader::new(file))
}

fn find_start(grid: &Vec<Vec<char>>) -> (usize, usize) {
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == '^' {
                return (row, col);
            }
        }
    }
    (0, 0)
}

fn move_guard(
    grid: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    direction: Direction,
) -> (usize, usize) {
    let mut new_row = row;
    let mut new_col = col;
    match direction {
        Direction::N => {
            if row > 0 && grid[row - 1][col] != '#' {
                new_row -= 1;
            }
        }
        Direction::S => {
            if row < grid.len() - 1 && grid[row + 1][col] != '#' {
                new_row += 1;
            }
        }
        Direction::E => {
            if col < grid[row].len() - 1 && grid[row][col + 1] != '#' {
                new_col += 1;
            }
        }
        Direction::W => {
            if col > 0 && grid[row][col - 1] != '#' {
                new_col -= 1;
            }
        }
    }
    (new_row, new_col)
}

pub fn first_star() -> Result<(), std::io::Error> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let reader = read_file().unwrap();
    let mut visited_positions = HashSet::new();

    for line in reader.lines() {
        let row = line?.chars().collect::<Vec<char>>();
        grid.push(row);
    }

    let (start_row, start_col) = find_start(&grid);
    let mut current_row = start_row;
    let mut current_col = start_col;
    let mut current_direction = Direction::N;

    visited_positions.insert((current_row, current_col));

    loop {
        let (next_row, next_col) = move_guard(&grid, current_row, current_col, current_direction);
        if next_row == current_row && next_col == current_col {
            current_direction = current_direction.rotate_clockwise();
        } else {
            current_row = next_row;
            current_col = next_col;
            visited_positions.insert((current_row, current_col));
        }

        if current_row == 0
            || current_row == grid.len() - 1
            || current_col == 0
            || current_col == grid[0].len() - 1
        {
            break;
        }
    }

    println!("Distinct Positions Visited: {}", visited_positions.len());
    Ok(())
}

pub fn second_star() -> Result<(), std::io::Error> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let reader = read_file()?;

    for line in reader.lines() {
        grid.push(line?.chars().collect());
    }

    let (start_row, start_col) = find_start(&grid);
    let mut loop_count = 0;

    // Try every position as potential obstacle sorry for the execution time :D
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if is_valid_loop(&mut grid, row, col, start_row, start_col) {
                loop_count += 1;
            }
        }
    }

    println!("Valid loop positions: {}", loop_count);
    Ok(())
}

fn is_valid_loop(
    grid: &mut Vec<Vec<char>>,
    test_row: usize,
    test_col: usize,
    start_row: usize,
    start_col: usize,
) -> bool {
    // Skip start & existing walls
    if (test_row == start_row && test_col == start_col) || grid[test_row][test_col] == '#' {
        return false;
    }

    let original = grid[test_row][test_col];
    grid[test_row][test_col] = '#';

    let mut visited = HashSet::new();
    let mut row = start_row;
    let mut col = start_col;
    let mut dir = Direction::N;

    loop {
        // Hit edge
        if row == 0 || row == grid.len() - 1 || col == 0 || col == grid[0].len() - 1 {
            grid[test_row][test_col] = original;
            return false;
        }

        // Already visited = loop
        if !visited.insert((row, col, dir)) {
            grid[test_row][test_col] = original;
            return true;
        }

        let (next_row, next_col) = move_guard(&grid, row, col, dir);

        if next_row == row && next_col == col {
            dir = dir.rotate_clockwise();
        } else {
            row = next_row;
            col = next_col;
        }
    }
}
