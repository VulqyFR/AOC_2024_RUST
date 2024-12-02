use aoc::day02;

fn main() -> std::io::Result<()> {
    println!("Advent of Code 2024 - Day 1 - https://adventofcode.com/2024/day/1 \n\n");
    println!("First star: \n");
    let start = std::time::Instant::now();
    day02::first_star()?;
    println!("Elapsed time: {:?}\n\n", start.elapsed());
    println!("Second star:\n");
    let start = std::time::Instant::now();
    day02::second_star()?;
    println!("Elapsed time: {:?}\n", start.elapsed());
    Ok(())
}
