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

fn parse_input(path: &str) -> Result<Vec<Vec<u8>>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader.lines().map(|s| s.unwrap().into_bytes()).collect())
}

fn problem_1(terrain: &[Vec<u8>]) {
    let rows = terrain.len();
    let cols = terrain[0].len();

    let mut terrain: Vec<Vec<_>> = terrain.to_vec();

    let mut dist: Vec<Vec<_>> = repeat(repeat(usize::MAX).take(cols).collect())
        .take(rows)
        .collect();
    let mut visited: Vec<Vec<_>> = repeat(repeat(false).take(cols).collect())
        .take(rows)
        .collect();
    let mut queue = VecDeque::new();

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

    // Set starting values.
    visited[sr][sc] = true;
    dist[sr][sc] = 0;
    queue.push_back((Position { row: sr, col: sc }, 0));

    while let Some((Position { row, col }, steps)) = queue.pop_front() {
        visited[row][col] = true;

        if row == gr && col == gc {
            // Todo print score.
            println!("Found end: {}", steps);
            break;
        }

        // Check whether we already have found a path to this position. If so, we can continue.
        if steps + 1 > dist[row][col] {
            continue;
        }

        // Find and add all adjacent edges.
        for (dr, dc) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
            let nr = (row as isize) + dr;
            let nc = (col as isize) + dc;

            if nr < 0 || nc < 0 {
                continue;
            }

            let nr = nr as usize;
            let nc = nc as usize;

            let Some(height) = terrain.get(nr).and_then(|r| r.get(nc)) else {continue; };

            if height.abs_diff(terrain[row][col]) <= 1 && steps + 1 < dist[nr][nc] {
                queue.push_back((Position { row: nr, col: nc }, steps + 1));

                dist[nr][nc] = steps + 1;
            }
        }
    }

    println!("Finished running");
}

fn main() -> Result<()> {
    let terrain = parse_input("example.txt")?;
    // let terrain = parse_input("input.txt")?;
    for row in &terrain {
        for col in row {
            print!("{}", *col as char);
        }
        println!();
    }
    println!();

    problem_1(&terrain);

    Ok(())
}
