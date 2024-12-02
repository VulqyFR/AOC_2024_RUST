mod day01;
mod day02;

fn main() -> std::io::Result<()> {
    println!("\n🎄 Welcome to my Advent of Code 2024 solutions in Rust 🦀!");
    println!("I hope you enjoy the code and learn something new with me \n\n");
    println!("Advent of Code 2024 - Day 1 - https://adventofcode.com/2024/day/1 \n");
    println!("First star: \n");
    let start = std::time::Instant::now();
    day01::first_star()?;
    println!("Elapsed time: {:?}\n\n", start.elapsed());
    println!("Second star:\n");
    let start = std::time::Instant::now();
    day01::second_star()?;
    println!("Elapsed time: {:?}\n", start.elapsed());

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
