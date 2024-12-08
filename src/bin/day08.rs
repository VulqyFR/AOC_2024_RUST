use aoc::day08;

fn main() -> std::io::Result<()> {
    println!("Advent of Code 2024 - Day 3 - https://adventofcode.com/2024/day/3 \n\n");
    println!("First star: \n");
    let start = std::time::Instant::now();
    day08::first_star()?;
    println!("Elapsed time: {:?}\n\n", start.elapsed());
    println!("Second star:\n");
    let start = std::time::Instant::now();
    day08::second_star()?;
    println!("Elapsed time: {:?}\n", start.elapsed());
    Ok(())
}
