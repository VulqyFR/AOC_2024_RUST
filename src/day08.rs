use std::collections::HashSet;

fn get_antennas(grid: &[Vec<char>]) -> Vec<(char, (i32, i32))> {
    let mut antennas = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c != '.' {
                antennas.push((c, (y as i32, x as i32)));
            }
        }
    }
    antennas
}

fn find_antinodes(pos1: (i32, i32), pos2: (i32, i32), rows: i32, cols: i32) -> Vec<(i32, i32)> {
    let (y1, x1) = pos1;
    let (y2, x2) = pos2;
    let dy = y2 - y1;
    let dx = x2 - x1;

    let candidates = vec![(y1 - dy, x1 - dx), (y2 + dy, x2 + dx)];

    candidates
        .into_iter()
        .filter(|&(y, x)| y >= 0 && y < rows && x >= 0 && x < cols) // Filter out out-of-bounds nodes
        .collect()
}

fn is_collinear(p1: (i32, i32), p2: (i32, i32), p3: (i32, i32)) -> bool {
    let (x1, y1) = (p1.1, p1.0);
    let (x2, y2) = (p2.1, p2.0);
    let (x3, y3) = (p3.1, p3.0);

    (y2 - y1) * (x3 - x1) == (y3 - y1) * (x2 - x1)
}

fn find_points_between(p1: (i32, i32), p2: (i32, i32), rows: i32, cols: i32) -> Vec<(i32, i32)> {
    let mut points = Vec::new();

    for y in 0..rows {
        for x in 0..cols {
            if is_collinear(p1, p2, (y, x)) {
                points.push((y, x));
            }
        }
    }
    points
}

pub fn first_star() -> Result<(), std::io::Error> {
    let input = std::fs::read_to_string("src/inputs/day08.txt")?;
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    let antennas = get_antennas(&grid);
    let mut antinodes = HashSet::new();

    for i in 0..antennas.len() {
        for j in i + 1..antennas.len() {
            if antennas[i].0 == antennas[j].0 {
                let nodes = find_antinodes(antennas[i].1, antennas[j].1, rows, cols);
                antinodes.extend(nodes);
            }
        }
    }

    println!("Number of unique antinodes ⭐ : {}", antinodes.len());
    Ok(())
}

pub fn second_star() -> Result<(), std::io::Error> {
    let input = std::fs::read_to_string("src/inputs/day08.txt")?;
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    let antennas = get_antennas(&grid);
    let mut antinodes = HashSet::new();

    for i in 0..antennas.len() {
        for j in i + 1..antennas.len() {
            if antennas[i].0 == antennas[j].0 {
                let points = find_points_between(antennas[i].1, antennas[j].1, rows, cols);
                antinodes.extend(points);
            }
        }
    }

    println!("Number of unique antinodes ⭐⭐ : {}", antinodes.len());
    Ok(())
}
