use aoc::day07;

fn main() -> std::io::Result<()> {
    println!("Advent of Code 2024 - Day 3 - https://adventofcode.com/2024/day/3 \n\n");
    println!("First star: \n");
    let start = std::time::Instant::now();
    day07::first_star()?;
    println!("Elapsed time: {:?}\n\n", start.elapsed());
    println!("Second star:\n");
    let start = std::time::Instant::now();
    day07::second_star()?;
    println!("Elapsed time: {:?}\n", start.elapsed());
    Ok(())
}
