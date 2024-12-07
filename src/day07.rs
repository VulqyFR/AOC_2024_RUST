use std::collections::HashMap;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn read_file() -> Result<BufReader<File>, std::io::Error> {
    let file = File::open("src/inputs/day07.txt")?;
    Ok(BufReader::new(file))
}

fn try_combinations(values: &[u64], target: u64) -> bool {
    if values.len() == 1 {
        return values[0] == target;
    }

    fn dfs(nums: &[u64], curr_val: u64, pos: usize, target: u64) -> bool {
        if pos == nums.len() {
            return curr_val == target;
        }

        if dfs(nums, curr_val + nums[pos], pos + 1, target) {
            return true;
        }

        if dfs(nums, curr_val * nums[pos], pos + 1, target) {
            return true;
        }

        false
    }

    dfs(&values[1..], values[0], 0, target)
}

fn try_combinations_with_concat(values: &[u64], target: u64) -> bool {
    if values.len() == 1 {
        return values[0] == target;
    }

    if try_combinations(values, target) {
        return true;
    }

    fn dfs(nums: &[u64], curr_val: u64, pos: usize, target: u64) -> bool {
        if pos == nums.len() {
            return curr_val == target;
        }

        let concat_str = format!("{}{}", curr_val, nums[pos]);
        if let Ok(concat_val) = concat_str.parse::<u64>() {
            if dfs(nums, concat_val, pos + 1, target) {
                return true;
            }
        }

        if dfs(nums, curr_val + nums[pos], pos + 1, target) {
            return true;
        }

        if dfs(nums, curr_val * nums[pos], pos + 1, target) {
            return true;
        }

        false
    }

    dfs(&values[1..], values[0], 0, target)
}

pub fn first_star() -> Result<(), std::io::Error> {
    let reader = read_file()?;
    let mut equations: HashMap<u64, Vec<u64>> = HashMap::new();
    let mut sum = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(":").collect();
        let key = match parts[0].trim().parse::<u64>() {
            Ok(key) => key,
            Err(_) => continue,
        };
        let values: Vec<u64> = parts[1]
            .split_whitespace()
            .filter_map(|s| s.parse::<u64>().ok())
            .collect();

        equations.insert(key, values);
    }
    for (key, values) in equations.iter() {
        if try_combinations(values, *key) {
            sum += key;
        }
    }
    println!("Sum of valid equations: {}", sum);
    Ok(())
}

pub fn second_star() -> Result<(), std::io::Error> {
    let reader = read_file()?;
    let mut equations: HashMap<u64, Vec<u64>> = HashMap::new();
    let mut sum = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(":").collect();
        let key = match parts[0].trim().parse::<u64>() {
            Ok(key) => key,
            Err(_) => continue,
        };
        let values: Vec<u64> = parts[1]
            .split_whitespace()
            .filter_map(|s| s.parse::<u64>().ok())
            .collect();

        equations.insert(key, values);
    }
    for (key, values) in equations.iter() {
        if try_combinations_with_concat(values, *key) {
            sum += key;
        }
    }
    println!("Total calibration result: {}", sum);
    Ok(())
}
