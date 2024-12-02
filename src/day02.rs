use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_numbers_from_file(path: &str) -> io::Result<Vec<Vec<i32>>> {
    let file = File::open(path)?;
    let buf_reader = BufReader::new(file);
    let mut result = Vec::new();

    for line in buf_reader.lines() {
        let nums: Vec<i32> = line?
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        result.push(nums);
    }

    Ok(result)
}

fn is_valid_sequence(nums: &[i32]) -> bool {
    if nums.len() < 2 {
        return false;
    }

    let increasing = nums[1] > nums[0];
    for i in 1..nums.len() {
        let diff = (nums[i] - nums[i - 1]).abs();
        if diff > 3
            || (increasing && nums[i] <= nums[i - 1])
            || (!increasing && nums[i] >= nums[i - 1])
        {
            return false;
        }
    }
    true
}

pub fn first_star() -> io::Result<()> {
    let sequences = read_numbers_from_file("src/inputs/day02.txt")?;
    let valid_count = sequences
        .iter()
        .filter(|nums| is_valid_sequence(nums))
        .count();

    println!("Valid: {}", valid_count);
    Ok(())
}

pub fn second_star() -> io::Result<()> {
    let sequences = read_numbers_from_file("src/inputs/day02.txt")?;
    let mut valid_count = 0;

    for sequence in sequences {
        if is_valid_sequence(&sequence) {
            valid_count += 1;
            continue;
        }

        // Test with Problem Dampener
        if (0..sequence.len()).any(|curr_idx| {
            let test_seq: Vec<_> = sequence
                .iter()
                .enumerate()
                .filter(|&(i, _)| i != curr_idx)
                .map(|(_, &x)| x)
                .collect();
            is_valid_sequence(&test_seq)
        }) {
            valid_count += 1;
        }
    }

    println!("Valid with dampener: {}", valid_count);
    Ok(())
}
