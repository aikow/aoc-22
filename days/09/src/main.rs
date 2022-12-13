use std::{
    collections::HashSet,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    iter::repeat,
    str::FromStr,
};

use anyhow::Result;

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Dir {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Dir::Up),
            "D" => Ok(Dir::Down),
            "L" => Ok(Dir::Left),
            "R" => Ok(Dir::Right),
            _ => Err(format!("Could not create direction from '{}'", s)),
        }
    }
}

impl Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Dir::Up => "Up",
                Dir::Down => "Down",
                Dir::Left => "Left",
                Dir::Right => "Right",
            }
        )?;

        Ok(())
    }
}

struct Inst {
    dir: Dir,
    count: i32,
}

impl Display for Inst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.dir, self.count)?;

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Loc {
    x: isize,
    y: isize,
}

fn main() -> Result<()> {
    // let instructions = parse_input("example.txt")?;
    // let instructions = parse_input("example2.txt")?;
    let instructions = parse_input("input.txt")?;

    // for inst in &instructions {
    //     println!("{}", inst);
    // }
    problem_1(&instructions);
    problem_2(&instructions);

    Ok(())
}

fn parse_input(path: &str) -> Result<Vec<Inst>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let instructions: Vec<_> = reader
        .lines()
        .map(|s| s.unwrap())
        .map(|line| {
            let mut parts = line.split_whitespace();
            let dir = Dir::from_str(parts.next().unwrap()).unwrap();
            let count = parts.next().unwrap().parse().unwrap();
            Inst { dir, count }
        })
        .collect();

    Ok(instructions)
}

fn too_far(a: &Loc, b: &Loc) -> bool {
    (a.x - b.x).abs() > 1 || (a.y - b.y).abs() > 1
}

fn problem_1(instructions: &[Inst]) {
    let mut head = Loc { x: 0, y: 0 };
    let mut tail = Loc { x: 0, y: 0 };

    let mut visited: HashSet<Loc> = HashSet::new();

    // Add current location;
    visited.insert(tail);

    for Inst { dir, count } in instructions {
        let dhead = match dir {
            Dir::Up => (1, 0),
            Dir::Down => (-1, 0),
            Dir::Left => (0, -1),
            Dir::Right => (0, 1),
        };

        // Iterate over the number of steps in that direction.
        for _ in 0..*count {
            // println!("Before moving: Head: ({}, {})", head.x, head.y);
            // Move the head.
            head.x += dhead.0;
            head.y += dhead.1;

            // Move the tail.
            if too_far(&head, &tail) {
                let normalised = (tail.x - head.x, tail.y - head.y);
                // println!("normalised: ({}, {})", normalised.0, normalised.1);
                let dtail = match normalised {
                    (0, 2) => (0, -1),
                    (0, -2) => (0, 1),

                    (2, 0) => (-1, 0),
                    (-2, 0) => (1, 0),

                    (x, 2) if x.abs() == 1 => (-x, -1),
                    (x, -2) if x.abs() == 1 => (-x, 1),

                    (2, y) if y.abs() == 1 => (-1, -y),
                    (-2, y) if y.abs() == 1 => (1, -y),

                    _ => (0, 0),
                };
                tail.x += dtail.0;
                tail.y += dtail.1;

                visited.insert(tail);
            }

            // println!(
            //     "After Moving: Head: ({}, {}), Tail: ({}, {})",
            //     head.x, head.y, tail.x, tail.y
            // );
        }
    }

    println!("Tail was in {} locations", visited.len());
}

fn problem_2(instructions: &[Inst]) {
    let mut rope: Vec<_> = repeat(Loc { x: 0, y: 0 }).take(10).collect();

    // Track the locations the last rope segment visited.
    let mut visited: Vec<HashSet<Loc>> = repeat(HashSet::new()).take(10).collect();

    // Add current location;
    for (idx, part) in rope.iter().enumerate() {
        visited[idx].insert(*part);
    }

    for Inst { dir, count } in instructions {
        let dhead = match dir {
            Dir::Up => (1, 0),
            Dir::Down => (-1, 0),
            Dir::Left => (0, -1),
            Dir::Right => (0, 1),
        };

        // Iterate over the number of steps in that direction.
        for _ in 0..*count {
            // println!("Before moving: Head: ({}, {})", head.x, head.y);
            // Move the head.
            let mut parts = rope.iter_mut().enumerate();
            let (idx, mut head) = parts.next().unwrap();
            visited[idx].insert(*head);
            head.x += dhead.0;
            head.y += dhead.1;

            for (idx, tail) in parts {
                // Move the tail.
                if too_far(head, tail) {
                    let nx = tail.x - head.x;
                    let ny = tail.y - head.y;

                    let (dx, dy) = match (nx, ny) {
                        (0, 0) => (0, 0),
                        (x, y) if x.abs() == y.abs() => {
                            (x.signum() * (x.abs() - 1), y.signum() * (y.abs() - 1))
                        }
                        (x, y) if x.abs() > y.abs() => (x.signum() * (x.abs() - 1), y),
                        (x, y) if x.abs() < y.abs() => (x, y.signum() * (y.abs() - 1)),
                        (x, y) => unreachable!("Unexpected motion: ({}, {})", x, y),
                    };
                    println!("(nx: {}, ny: {}) => (dx: {}, dy: {})", nx, ny, -dx, -dy);

                    tail.x -= dx;
                    tail.y -= dy;
                }

                visited[idx].insert(*tail);
                head = tail;
            }
            // println!(
            //     "After Moving: Head: ({}, {}), Tail: ({}, {})",
            //     head.x, head.y, tail.x, tail.y
            // );
        }
    }

    println!("Tail was in {} locations", visited[9].len());
}
