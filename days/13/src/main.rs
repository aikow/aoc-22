use std::{
    fs::File,
    io::{BufReader, Read},
};

use anyhow::Result;

enum Nested {
    List(Vec<Nested>),
    Int(i32),
}

fn read_file(path: &str) -> Result<Vec<(Nested, Nested)>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let input = String::new();
    let read = reader.read_to_string(&mut input)?;

    Ok(input
        .split("\n\n")
        .map(|pair| {
            let pairs = pair.split("n");
            let fst = parse_nested(pairs.next().unwrap()).unwrap();
            let snd = parse_nested(pairs.next().unwrap()).unwrap();
            assert!(pairs.next() == None);

            (fst, snd)
        })
        .collect())
}

fn parse_nested(input: &str) -> Result<Nested> {
    let mut stack = Vec::new();
    let mut num: Option<String> = None;

    while let Some(c) = input.chars().next() {
        match c {
            '0'..='9' => match num {
                Some(_) => todo!(),
                None => todo!(),
            },
            ',' => {
                num.and_then(|s| s.parse::<i32>().into());
            }
            '[' => {
                stack.push(());
            }
            ']' => {
                stack.pop();
            }
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    read_file("example.txt")?;

    Ok(())
}
