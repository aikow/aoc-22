use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use anyhow::Result;

struct Range {
    lower: i32,
    upper: i32,
}

fn main() -> Result<()> {
    let input = parse_input()?;
    problem_1(&input)?;
    problem_2(&input)?;

    Ok(())
}

fn parse_input() -> Result<Vec<(Range, Range)>> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);
    Ok(reader
        .lines()
        .map(|s| s.unwrap())
        .map(|line| {
            let parts: Vec<i32> = line
                .split(',')
                .flat_map(|p| p.split('-'))
                .map(|s| s.parse().unwrap())
                .collect();
            match &parts[..4] {
                &[a_low, a_high, b_low, b_high] => (
                    Range {
                        lower: a_low,
                        upper: a_high,
                    },
                    Range {
                        lower: b_low,
                        upper: b_high,
                    },
                ),
                _ => panic!("Bad line format"),
            }
        })
        .collect())
}

fn problem_1(input: &[(Range, Range)]) -> Result<()> {
    let num = input
        .iter()
        .filter(|(a, b)| {
            (a.lower <= b.lower && a.upper >= b.upper) || (b.lower <= a.lower && b.upper >= a.upper)
        })
        .count();

    println!("Number of contained sets: {}", num);
    Ok(())
}
fn problem_2(input: &[(Range, Range)]) -> Result<()> {
    let num = input
        .iter()
        .filter(|(a, b)| {
            (b.upper >= a.lower && b.upper <= a.upper) || (a.upper >= b.lower && a.upper <= b.upper)
        })
        .count();

    println!("Number of overlapping sets: {}", num);
    Ok(())
}
