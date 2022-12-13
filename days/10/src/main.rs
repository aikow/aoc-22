use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use anyhow::Result;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Noop,
    Addx { num: i32 },
}

impl Instruction {
    fn cycles(&self) -> i32 {
        match self {
            Instruction::Noop => 1,
            Instruction::Addx { num: _ } => 2,
        }
    }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let cmd = parts.next().unwrap();

        match cmd {
            "noop" => Ok(Instruction::Noop),
            "addx" => Ok(Instruction::Addx {
                num: parts.next().unwrap().parse().unwrap(),
            }),
            x => Err(format!("Failed to parse {} as a command", x)),
        }
    }
}

fn parse_input(path: &str) -> Result<Vec<Instruction>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader
        .lines()
        .map(|s| Instruction::from_str(s.unwrap().as_str()).unwrap())
        .collect())
}

fn problem_1(instructions: &[Instruction]) {
    const COUNTER_SCORES: [i32; 6] = [20, 60, 100, 140, 180, 220];
    let mut score = 0;

    let mut program_counter = 0;
    let mut insts = instructions.iter();
    let mut inst = insts.next().unwrap();
    let mut inst_cycles = inst.cycles();

    // Registers
    let mut register_x = 1;

    loop {
        // Increment program counter at the end.
        program_counter += 1;

        // println!(
        //     "LOOP - PC: {} INST: {:?} CYC: {} SCORE: {}",
        //     program_counter, inst, inst_cycles, score
        // );

        // check whether we care about the current counter.
        if COUNTER_SCORES.contains(&program_counter) {
            score += program_counter * register_x;
        }

        // Check whether the current command still has cycles left.
        if inst_cycles > 1 {
            inst_cycles -= 1;
            continue;
        }

        // If command is finished, process it.
        match inst {
            Instruction::Noop => (),
            Instruction::Addx { num } => register_x += num,
        }

        // Get next instruction
        let Some(i) = insts.next() else {
            break;
        };
        inst = i;
        inst_cycles = i.cycles();
    }

    println!("Problem 1: {}", score);
}

fn problem_2(instructions: &[Instruction]) {
    const COUNTER_SCORES: [i32; 6] = [20, 60, 100, 140, 180, 220];
    let mut score = 0;

    let mut program_counter = 0;
    let mut insts = instructions.iter();
    let mut inst = insts.next().unwrap();
    let mut inst_cycles = inst.cycles();

    let mut crt: Vec<bool> = Vec::with_capacity(240);

    // Registers
    let mut register_x: i32 = 1;

    loop {
        // Increment program counter at the end.
        program_counter += 1;


        // println!(
        //     "LOOP - PC: {} INST: {:?} CYC: {} SCORE: {}",
        //     program_counter, inst, inst_cycles, score
        // );

        // Check whether sprite is visible.
        let x_pos = (program_counter - 1) % 40;
        println!("{} - {}", x_pos, register_x);
        crt.push(register_x.abs_diff(x_pos) <= 1);

        // Check whether the current command still has cycles left.
        if inst_cycles > 1 {
            inst_cycles -= 1;
            continue;
        }

        // If command is finished, process it.
        match inst {
            Instruction::Noop => (),
            Instruction::Addx { num } => register_x += num,
        }

        // Get next instruction
        let Some(i) = insts.next() else {
            break;
        };
        inst = i;
        inst_cycles = i.cycles();
    }

    println!("Problem 2");
    for (idx, pixel) in crt.iter().enumerate() {
        if idx % 40 == 0 {
            println!();
        }
        print!("{}", if *pixel { "#" } else { " " });
    }
}
fn main() -> Result<()> {
    let commands = parse_input("input.txt")?;
    problem_1(&commands);
    problem_2(&commands);

    Ok(())
}
