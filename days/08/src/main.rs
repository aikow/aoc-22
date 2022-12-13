use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    iter::repeat,
};

use anyhow::Result;
use itertools::Itertools;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() -> Result<()> {
    let forest = parse_input("input.txt")?;
    println!("{}", forest.iter().map(|r| r.iter().join("")).join("\n"));

    problem_1(&forest)?;
    problem_2(&forest)?;

    Ok(())
}

fn parse_input(path: &str) -> Result<Vec<Vec<i8>>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let forest: Vec<Vec<i8>> = reader
        .lines()
        .map(|s| s.unwrap().bytes().map(|b| (b - b'0') as i8).collect())
        .collect();

    Ok(forest)
}

fn add_visible(
    forest: &[Vec<i8>],
    visible: &mut HashSet<(usize, usize)>,
    mut row: isize,
    mut col: isize,
    direction: Direction,
) {
    let mut prev = -1;
    let rows = forest.len() as isize;
    let cols = forest[0].len() as isize;
    while row >= 0 && row < rows && col >= 0 && col < cols {
        let tree = forest[row as usize][col as usize];
        if prev < tree {
            visible.insert((row as usize, col as usize));
            prev = tree;
        }

        match direction {
            Direction::Up => row -= 1,
            Direction::Down => row += 1,
            Direction::Left => col -= 1,
            Direction::Right => col += 1,
        }
    }
}

fn problem_1(forest: &[Vec<i8>]) -> Result<()> {
    let mut visible = HashSet::new();

    let rows = forest.len();
    let cols = forest[0].len();

    for row in 0..rows {
        add_visible(forest, &mut visible, row as isize, 0, Direction::Right);
        add_visible(
            forest,
            &mut visible,
            row as isize,
            (cols as isize) - 1,
            Direction::Left,
        );
    }

    for col in 0..cols {
        add_visible(forest, &mut visible, 0, col as isize, Direction::Down);
        add_visible(
            forest,
            &mut visible,
            (rows as isize) - 1,
            col as isize,
            Direction::Up,
        );
    }

    println!();
    println!(
        "{}",
        forest
            .iter()
            .enumerate()
            .map(|(r, row)| row
                .iter()
                .enumerate()
                .map(|(c, t)| if visible.contains(&(r, c)) {
                    format!("{}", t)
                } else {
                    String::from(" ")
                })
                .join(""))
            .join("\n")
    );
    println!("Number of trees visible from edge: {}", visible.len());

    Ok(())
}

fn add_visible_trees(forest: &[Vec<i8>], row: usize, col: usize, direction: Direction) -> i32 {
    let rows = forest.len();
    let cols = forest[0].len();

    let range: Box<dyn Iterator<Item = (usize, usize)>> = match direction {
        Direction::Up => Box::new((row+1..rows).zip(repeat(col))),
        Direction::Down => Box::new((0..row).rev().zip(repeat(col))),
        Direction::Left => Box::new(repeat(row).zip(col+1..cols)),
        Direction::Right => Box::new(repeat(row).zip((0..col).rev())),
    };

    let orig = forest[row][col];
    let mut count = 0;
    for (r, c) in range {
        // println!("looping ({}, {})", r, c);
        count += 1;
        if forest[r][c] >= orig {
            break;
        }
    }
    // println!(
    //     "({}, {}) has {} visible tree's to the {:?}",
    //     row, col, count, direction
    // );
    count
}

fn problem_2(forest: &[Vec<i8>]) -> Result<()> {
    let mut scores: HashMap<(usize, usize), i32> = HashMap::new();

    let rows = forest.len();
    let cols = forest[0].len();

    for row in 0..rows {
        for col in 0..cols {
            let mut count = 1;
            count *= add_visible_trees(forest, row, col, Direction::Up);
            count *= add_visible_trees(forest, row, col, Direction::Down);
            count *= add_visible_trees(forest, row, col, Direction::Left);
            count *= add_visible_trees(forest, row, col, Direction::Right);
            scores.insert((row, col), count);
        }
    }

    let mut sorted_scores: Vec<_> = scores.into_iter().collect();
    sorted_scores.sort_by_key(|(_, v)| *v);
    println!("Problem 2: {:?}", sorted_scores.last().unwrap().1);

    Ok(())
}
