use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Result;

fn main() -> Result<()> {
    let calories = parse_input()?;
    for elf in &calories {
        println!("{:?}", elf);
    }

    problem_1(&calories)?;
    problem_2(&calories)?;

    Ok(())
}

fn parse_input() -> Result<Vec<Vec<i32>>> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);

    let mut calories = Vec::new();
    let mut elf = Vec::new();
    for line in reader.lines().map(|s| s.unwrap()) {
        if line.is_empty() {
            calories.push(elf);
            elf = Vec::new();
        } else {
            elf.push(line.parse().unwrap());
        }
    }

    Ok(calories)
}

fn problem_1(calories: &[Vec<i32>]) -> Result<()> {
    let elf: i32 = calories.iter().map(|elf| {
        elf.iter().sum()
    }).max().unwrap();

    println!("Problem 1: {}", elf);
    
    Ok(())
}

fn problem_2(calories: &[Vec<i32>]) -> Result<()> {
    let mut elves: Vec<i32> = calories.iter().map(|elf| {
        elf.iter().sum()
    }).collect();

    elves.sort();

    let calories: i32 = elves.iter().rev().take(3).sum();

    println!("Problem 2: {}", calories);
    
    Ok(())
}
