use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;

struct MoveInstructions {
    num: usize,
    from: usize,
    to: usize,
}

fn main() -> Result<()> {
    let (stacks, moves) = parse_input()?;
    problem_1(&stacks, &moves);
    problem_2(&stacks, &moves);

    Ok(())
}

fn problem_1(stacks: &[Vec<char>], moves: &[MoveInstructions]) {
    let mut stacks: Vec<_> = stacks.to_vec();

    for mv in moves {
        for _ in 0..mv.num {
            match stacks[mv.from].pop() {
                Some(val) => stacks[mv.to].push(val),
                None => panic!("Tried to pop from empty stack {}.", mv.from),
            }
        }
    }

    for (i, stack) in stacks.iter().enumerate() {
        println!("{}: {:?}", i, stack);
    }

    let res: String = stacks.iter().map(|s| s.last().unwrap()).collect();
    println!("Result: {}", res);
}

fn problem_2(stacks: &[Vec<char>], moves: &[MoveInstructions]) {
    let mut stacks: Vec<_> = stacks.to_vec();

    for mv in moves {
        let idx = stacks[mv.from].len() - mv.num;
        let mut elems: Vec<char> = stacks[mv.from].iter().skip(idx).copied().collect();
        stacks[mv.to].append(&mut elems);
        for _ in 0..mv.num {
            stacks[mv.from].pop();
        }
    }

    for (i, stack) in stacks.iter().enumerate() {
        println!("{}: {:?}", i, stack);
    }

    let res: String = stacks.iter().map(|s| s.last().unwrap()).collect();
    println!("Result: {}", res);
}

fn parse_input() -> Result<(Vec<Vec<char>>, Vec<MoveInstructions>)> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);
    let lines: Vec<_> = reader.lines().map(|s| s.unwrap()).collect();

    let mut stack_lines = Vec::new();
    let mut move_lines = Vec::new();
    let mut move_inst = false;
    for line in lines {
        if line.is_empty() {
            move_inst = true;
            continue;
        }
        match move_inst {
            false => stack_lines.push(line),
            true => move_lines.push(line),
        }
    }

    stack_lines.reverse();
    let mut stack_lines_iter = stack_lines.into_iter();
    let mut stacks: Vec<_> = stack_lines_iter
        .next()
        .unwrap()
        .split_whitespace()
        .map(|_| Vec::new())
        .collect();

    for line in stack_lines_iter {
        for (i, stack) in stacks.iter_mut().enumerate() {
            if i * 4 + 1 >= line.len() {
                break;
            }
            let byte = line.as_bytes()[i * 4 + 1];
            if let b'A'..=b'Z' = byte {
                stack.push(char::from(byte))
            }
        }
    }

    lazy_static! {
        static ref RE_MOVE: Regex =
            Regex::new(r"move (?P<num>\d+) from (?P<from>\d+) to (?P<to>\d+)").unwrap();
    }

    let move_instructions = move_lines
        .into_iter()
        .map(|line| {
            let Some(caps) = RE_MOVE.captures(&line) else {
                panic!("Failed to parse \"{}\"", line);
            };
            MoveInstructions {
                num: caps["num"].parse().unwrap(),
                from: caps["from"].parse::<usize>().unwrap() - 1,
                to: caps["to"].parse::<usize>().unwrap() - 1,
            }
        })
        .collect();

    for stack in &stacks {
        println!("{:?}", stack);
    }

    Ok((stacks, move_instructions))
}
