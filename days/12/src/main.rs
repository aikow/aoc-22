use std::collections::VecDeque;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::iter::repeat;

use anyhow::Result;
use itertools::Itertools;

#[derive(Clone, Copy, Eq, PartialEq)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn add(&self, dr: isize, dc: isize, rows: usize, cols: usize) -> Option<Self> {
        let row = self.row as isize + dr;
        let col = self.col as isize + dc;

        if row < 0 || col < 0 || row as usize >= rows || col as usize >= cols {
            None
        } else {
            Some(Position {
                row: row as usize,
                col: col as usize,
            })
        }
    }
}

fn parse_input(path: &str) -> Result<Vec<Vec<u8>>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader.lines().map(|s| s.unwrap().into_bytes()).collect())
}

/// Perform a breadth-first search from start and return the number of steps it takes to reach end.
///
/// Only steps where the next step is at most 1 higher than the current elevation are allowed.
fn bfs(grid: &[Vec<u8>], start: Position, end: Position) -> Option<usize> {
    let rows = grid.len();
    let cols = grid[0].len();

    // Keep track of distance to each node.
    let mut dist: Vec<Vec<_>> = repeat(repeat(usize::MAX).take(cols).collect())
        .take(rows)
        .collect();

    // Track the breadth-first search in a queue, which stores the next position and the current
    // depth from the start node.
    let mut queue = VecDeque::new();

    // Set starting values.
    dist[start.row][start.col] = 0;
    queue.push_back((start, 0));

    while let Some((position, depth)) = queue.pop_front() {
        if position == end {
            return Some(depth);
        }

        // Check whether we already have found a path to this position. If so, we can continue.
        if depth + 1 < dist[position.row][position.col] {
            continue;
        }

        let height = grid[position.row][position.col];

        // Find and add all adjacent edges.
        for (dr, dc) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
            let Some(next) = position.add(dr, dc, rows, cols) else { continue; };
            let next_height = grid[next.row][next.col];

            if (next_height < height || next_height - height <= 1)
                && depth + 1 < dist[next.row][next.col]
            {
                queue.push_back((next, depth + 1));
                dist[next.row][next.col] = depth + 1;
            }
        }
    }

    None
}

/// Perform a breadth-first search from starting position until we reach an elevation of 'a'.
fn reverse_bfs(grid: &[Vec<u8>], start: Position) -> Option<usize> {
    let rows = grid.len();
    let cols = grid[0].len();

    // Keep track of distance to each node.
    let mut dist: Vec<Vec<_>> = repeat(repeat(usize::MAX).take(cols).collect())
        .take(rows)
        .collect();

    // Track the breadth-first search in a queue, which stores the next position and the current
    // depth from the start node.
    let mut queue = VecDeque::new();

    // Set starting values.
    dist[start.row][start.col] = 0;
    queue.push_back((start, 0));

    while let Some((position, depth)) = queue.pop_front() {
        let height = grid[position.row][position.col];

        if height == b'a' {
            return Some(depth);
        }

        // Check whether we already have found a path to this position. If so, we can continue.
        if depth + 1 < dist[position.row][position.col] {
            continue;
        }

        // Find and add all adjacent edges.
        for (dr, dc) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
            let Some(next) = position.add(dr, dc, rows, cols) else { continue; };
            let next_height = grid[next.row][next.col];

            if (next_height > height || height - next_height <= 1)
                && depth + 1 < dist[next.row][next.col]
            {
                queue.push_back((next, depth + 1));
                dist[next.row][next.col] = depth + 1;
            }
        }
    }

    None
}

fn problem_1(terrain: &[Vec<u8>], start: Position, end: Position) {
    match bfs(terrain, start, end) {
        Some(steps) => println!("Found end in {} steps", steps),
        None => println!("End was not reachable from start"),
    }
}

fn problem_2(terrain: &[Vec<u8>], start: Position) {
    match reverse_bfs(terrain, start) {
        Some(steps) => println!("Elevation of 'a' was reachable in {} steps", steps),
        None => println!("Unable to reach elevation 'a'"),
    }
}

fn main() -> Result<()> {
    // let terrain = parse_input("example.txt")?;
    let mut terrain = parse_input("input.txt")?;

    let rows = terrain.len();
    let cols = terrain[0].len();

    // Find start and end positions.
    let (sr, sc) = (0..rows)
        .cartesian_product(0..cols)
        .find(|&(x, y)| terrain[x][y] == b'S')
        .unwrap();
    let (gr, gc) = (0..rows)
        .cartesian_product(0..cols)
        .find(|&(x, y)| terrain[x][y] == b'E')
        .unwrap();

    // Update the height map with the actual values of S and E.
    terrain[sr][sc] = b'a';
    terrain[gr][gc] = b'z';

    let start = Position { row: sr, col: sc };
    let end = Position { row: gr, col: gc };

    problem_1(&terrain, start, end);
    problem_2(&terrain, end);

    Ok(())
}
