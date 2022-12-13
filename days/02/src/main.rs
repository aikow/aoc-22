use anyhow::Result;
use std::cmp::Ordering;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Eq, PartialEq, Debug)]
enum Ending {
    Lose,
    Tie,
    Win,
}

impl TryFrom<&str> for Ending {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Tie),
            "Z" => Ok(Self::Win),
            _ => Err(format!("Could not create Ending from {}", value)),
        }
    }
}

#[derive(Eq, PartialEq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn value(&self) -> i32 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }

    fn for_outcome(&self, ending: &Ending) -> Self {
        match (self, ending) {
            (Choice::Rock, Ending::Tie) => Choice::Rock,
            (Choice::Rock, Ending::Lose) => Choice::Scissors,
            (Choice::Rock, Ending::Win) => Choice::Paper,
            (Choice::Paper, Ending::Tie) => Choice::Paper,
            (Choice::Paper, Ending::Lose) => Choice::Rock,
            (Choice::Paper, Ending::Win) => Choice::Scissors,
            (Choice::Scissors, Ending::Tie) => Choice::Scissors,
            (Choice::Scissors, Ending::Lose) => Choice::Paper,
            (Choice::Scissors, Ending::Win) => Choice::Rock,
        }
    }
}

impl TryFrom<&str> for Choice {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" => Ok(Choice::Rock),
            "B" => Ok(Choice::Paper),
            "C" => Ok(Choice::Scissors),
            "X" => Ok(Choice::Rock),
            "Y" => Ok(Choice::Paper),
            "Z" => Ok(Choice::Scissors),
            _ => Err(format!("Could not create Choice from {}", value)),
        }
    }
}

impl Display for Choice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Choice::Rock => "R",
            Choice::Paper => "P",
            Choice::Scissors => "S",
        })
    }
}

impl PartialOrd for Choice {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(match (self, other) {
            (Choice::Rock, Choice::Rock) => Ordering::Equal,
            (Choice::Rock, Choice::Paper) => Ordering::Less,
            (Choice::Rock, Choice::Scissors) => Ordering::Greater,
            (Choice::Paper, Choice::Rock) => Ordering::Greater,
            (Choice::Paper, Choice::Paper) => Ordering::Equal,
            (Choice::Paper, Choice::Scissors) => Ordering::Less,
            (Choice::Scissors, Choice::Rock) => Ordering::Less,
            (Choice::Scissors, Choice::Paper) => Ordering::Greater,
            (Choice::Scissors, Choice::Scissors) => Ordering::Equal,
        })
    }
}

impl Ord for Choice {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn main() -> Result<()> {
    let choices = parse_input()?;

    println!("Elements: {}", choices.len());
    // for (a, b) in &choices {
    //     println!("{} {}", a, b);
    // }

    problem_1(&choices)?;
    problem_2(&choices)?;

    Ok(())
}
fn parse_input() -> Result<Vec<(Choice, String)>> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);

    let mut choices = Vec::new();

    for line in reader.lines().map(|s| s.unwrap()) {
        let mut parts = line.split_whitespace();

        let first = Choice::try_from(parts.next().unwrap()).unwrap();
        let second = String::from(parts.next().unwrap());
        choices.push((first, second));
    }

    Ok(choices)
}

fn problem_1(choices: &[(Choice, String)]) -> Result<()> {
    let score: i32 = choices
        .iter()
        .map(|(c, s)| (c, Choice::try_from(&s[..]).unwrap()))
        .map(|(oponent, choice)| {
            choice.value()
                + match choice.cmp(oponent) {
                    Ordering::Less => 0,
                    Ordering::Equal => 3,
                    Ordering::Greater => 6,
                }
        })
        .sum();

    println!("Total score: {}", score);

    Ok(())
}

fn problem_2(choices: &[(Choice, String)]) -> Result<()> {
    let score: i32 = choices
        .iter()
        .map(|(c, s)| (c, Ending::try_from(&s[..]).unwrap()))
        .map(|(oponent, ending)| {
            let choice = oponent.for_outcome(&ending);
            choice.value()
                + match choice.cmp(oponent) {
                    Ordering::Less => 0,
                    Ordering::Equal => 3,
                    Ordering::Greater => 6,
                }
        })
        .sum();

    println!("Total score: {}", score);

    Ok(())
}
