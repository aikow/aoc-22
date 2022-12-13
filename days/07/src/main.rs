use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let sizes = parse_input("input.txt")?;
    let sum: usize = sizes.values().filter(|s| **s <= 100000).sum();
    println!("Problem 1: {}", sum);

    // Problem 2
    const FS_SIZE: usize = 70000000;
    const UPDATE_SIZE: usize = 30000000;
    let used_size = sizes["/"];
    let free_size = dbg!(FS_SIZE - used_size);
    let missing_size = dbg!(UPDATE_SIZE - free_size);
    let mut sizes_2: Vec<_> = sizes.values().filter(|s| **s >= missing_size).collect();
    sizes_2.sort();
    println!(
        "Problem 2: {} {}",
        sizes_2.first().unwrap(),
        sizes_2.last().unwrap()
    );

    Ok(())
}

fn parse_input(path: &str) -> Result<HashMap<String, usize>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Stack of directories, representing current working dir.
    let mut pwd: Vec<String> = Vec::new();
    let mut sizes: HashMap<String, usize> = HashMap::new();

    for s in reader.lines() {
        let line = s.unwrap();
        let tokens: Vec<&str> = line.split_whitespace().collect();

        if tokens[0] == "$" {
            // If the line is a command.
            match tokens[1] {
                "cd" => {
                    let dirname = tokens[2];
                    match dirname {
                        ".." => {
                            // println!("Going up");
                            pwd.pop();
                        }
                        "/" => {
                            // println!("Going to root");
                            pwd.clear();
                            pwd.push(String::from("/"));
                        }
                        x => {
                            // println!("Going into {}", x);
                            pwd.push(String::from(x));
                        }
                    }
                }
                "ls" => (),
                _ => panic!("Shouldn't happen"),
            }
        } else {
            // Is a file system item.
            if tokens[0] != "dir" {
                let size: usize = tokens[0].parse().unwrap();
                let mut acc: Vec<&String> = Vec::with_capacity(pwd.len());
                for p in &pwd {
                    acc.push(p);
                    let full_path = acc.iter().join("/");
                    let entry = sizes.entry(full_path).or_default();
                    *entry += size;
                }
            }
        }
    }

    Ok(sizes)
}
