use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use anyhow::Result;
use itertools::Itertools;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() -> Result<()> {
    problem_1()?;
    problem_2()?;

    Ok(())
}

fn problem_1() -> Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);
    let score: i32 = reader
        .lines()
        .map(|line| {
            let mut s = line.unwrap();
            assert!(s.len() % 2 == 0);
            let t = s.split_off(s.len() / 2);
            let s_set: HashSet<char> = HashSet::from_iter(s.chars());
            let t_set: HashSet<char> = HashSet::from_iter(t.chars());
            let shared = s_set.intersection(&t_set).into_iter().next().unwrap();
            let val = ALPHABET.find(*shared).unwrap();
            val as i32 + 1
        })
        .sum();

    println!("Problem 1: {}", score);

    Ok(())
}

fn problem_2() -> Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);
    let score: i32 = reader
        .lines()
        .map(|s| s.unwrap())
        .chunks(3)
        .into_iter()
        .map(|mut chunk| {
            let s = chunk.next().unwrap();
            let t = chunk.next().unwrap();
            let u = chunk.next().unwrap();

            let s_set: HashSet<_> = HashSet::from_iter(s.chars());
            let t_set: HashSet<_> = HashSet::from_iter(t.chars());
            let u_set: HashSet<_> = HashSet::from_iter(u.chars());
            let s_t_shared: HashSet<_> = s_set.intersection(&t_set).cloned().collect();
            let id = s_t_shared.intersection(&u_set).into_iter().next().unwrap();
            let val = ALPHABET.find(*id).unwrap();
            val as i32 + 1
        })
        .sum();

    println!("Problem 1: {}", score);

    Ok(())
}
