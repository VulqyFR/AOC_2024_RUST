use std::collections::{HashMap, HashSet};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn is_valid_order(rules: &HashMap<u32, HashSet<u32>>, update: &[u32]) -> bool {
    for i in 0..update.len() {
        for j in i + 1..update.len() {
            let before = update[i];
            let after = update[j];

            if let Some(must_be_before) = rules.get(&after) {
                if must_be_before.contains(&before) {
                    return false;
                }
            }
        }
    }
    true
}

fn read_file(
    filename: &str,
) -> Result<(HashMap<u32, HashSet<u32>>, Vec<Vec<u32>>), std::io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut rules: HashMap<u32, HashSet<u32>> = HashMap::new();
    let mut updates: Vec<Vec<u32>> = Vec::new();
    let mut parsing_rules = true;

    for line in reader.lines() {
        let line = line?.trim().to_string();
        if line.is_empty() {
            parsing_rules = false;
            continue;
        }
        if parsing_rules {
            let parts: Vec<&str> = line.split('|').collect();
            let before: u32 = parts[0].parse().unwrap();
            let after: u32 = parts[1].parse().unwrap();

            rules
                .entry(before)
                .or_insert_with(HashSet::new)
                .insert(after);
            continue;
        }
        let update: Vec<u32> = line.split(',').map(|x| x.trim().parse().unwrap()).collect();
        updates.push(update);
    }
    Ok((rules, updates))
}

pub fn first_star() -> Result<(), std::io::Error> {
    let (rules, updates) = read_file("src/inputs/day05.txt")?;
    let mut sum = 0;

    for update in updates {
        if is_valid_order(&rules, &update) {
            let mid_index = update.len() / 2;
            sum += update[mid_index];
        }
    }

    println!("Sum : {}", sum);
    Ok(())
}

pub fn second_star() -> Result<(), std::io::Error> {
    let (rules, updates) = read_file("src/inputs/day05.txt")?;
    let mut sum = 0;

    for update in updates {
        if !is_valid_order(&rules, &update) {
            let mut sorted = update.clone();
            sorted.sort_by(|&a, &b| {
                if let Some(must_be_before) = rules.get(&b) {
                    if must_be_before.contains(&a) {
                        return std::cmp::Ordering::Greater;
                    }
                }
                if let Some(must_be_before) = rules.get(&a) {
                    if must_be_before.contains(&b) {
                        return std::cmp::Ordering::Less;
                    }
                }
                std::cmp::Ordering::Equal
            });
            let mid_index = sorted.len() / 2;
            sum += sorted[mid_index];
        }
    }

    println!("Sum : {}", sum);
    Ok(())
}
