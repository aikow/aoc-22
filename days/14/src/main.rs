use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Elem {
    Rock,
    Sand,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

const START: Point = Point { x: 500, y: 0 };

/// Store the rocks as a mapping from the horizontal position to all the rocks in that position.
type Cave = HashMap<i32, HashMap<i32, Elem>>;

fn parse_input(path: &str) -> anyhow::Result<Cave> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut cave = HashMap::new();

    for line in reader.lines().map(Result::unwrap) {
        let mut points = line.split(" -> ").map(|s| {
            let (x, y) = s.split_once(',').unwrap();
            (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
        });

        // There should be at least two elements in the list of points.
        let (mut px, mut py) = points.next().unwrap();
        for (cx, cy) in points {
            // Either x or y have to be the same;
            if px == cx {
                let set: &mut HashMap<i32, Elem> = cave.entry(px).or_default();
                for y in (std::cmp::min(py, cy))..=(std::cmp::max(py, cy)) {
                    set.insert(y, Elem::Rock);
                }
            } else if py == cy {
                for x in (std::cmp::min(px, cx))..=(std::cmp::max(px, cx)) {
                    let set: &mut HashMap<i32, Elem> = cave.entry(x).or_default();
                    set.insert(py, Elem::Rock);
                }
            } else {
                panic!("Either x or y should be equal");
            }

            (px, py) = (cx, cy);
        }
    }

    Ok(cave)
}
fn drop_sand(cave: &mut Cave, sand: Point) -> Option<Point> {
    // println!("Testing sand ({}, {})", sand.x, sand.y);
    let Some(vert) = cave.get(&sand.x) else { return None };
    let Some(y) = vert.keys().filter(|&&k| k > sand.y).min() else { return None };
    // println!("Sand can fall to {}", y - 1);

    // Check if left, then right fields are blocked.
    if let Some(left) = cave.get(&(sand.x - 1)) {
        // if left.contains_key(y) || left.contains_key(&(y - 1)) {
        if left.contains_key(y) {
            if let Some(right) = cave.get(&(sand.x + 1)) {
                // if right.contains_key(y) || right.contains_key(&(y - 1)) {
                if right.contains_key(y) {
                    // println!("Sand is blocked both left and right.");
                    Some(Point {
                        x: sand.x,
                        y: *y - 1,
                    })
                } else {
                    // println!("Testing whether sand can fall right.");
                    drop_sand(
                        cave,
                        Point {
                            x: sand.x + 1,
                            y: *y,
                        },
                    )
                }
            } else {
                // println!("Sand will fall into the abyss to the right.");
                None
            }
        } else {
            // println!("Testing whether sand can fall left.");
            drop_sand(
                cave,
                Point {
                    x: sand.x - 1,
                    y: *y,
                },
            )
        }
    } else {
        // println!("Sand will fall into the abyss to the left.");
        None
    }
}
/// Return true if the sand can be placed without dropping it.
fn drop_sand_with_bottom(cave: &mut Cave, sand: Point, bottom: i32) -> Point {
    // println!("Testing sand ({}, {})", sand.x, sand.y);
    let Some(vert) = cave.get(&sand.x) else { return Point { x: sand.x, y: bottom - 1 }};
    let Some(y) = vert.keys().filter(|&&k| k > sand.y).min() else { return Point { x: sand.x, y: bottom - 1} };
    // println!("Sand can fall to {}", y - 1);

    // Check if left, then right fields are blocked.
    if let Some(left) = cave.get(&(sand.x - 1)) {
        // if left.contains_key(y) || left.contains_key(&(y - 1)) {
        if left.contains_key(y) {
            if let Some(right) = cave.get(&(sand.x + 1)) {
                // if right.contains_key(y) || right.contains_key(&(y - 1)) {
                if right.contains_key(y) {
                    // println!("Sand is blocked both left and right.");
                    Point {
                        x: sand.x,
                        y: *y - 1,
                    }
                } else {
                    // println!("Testing whether sand can fall right.");
                    drop_sand_with_bottom(
                        cave,
                        Point {
                            x: sand.x + 1,
                            y: *y,
                        },
                        bottom,
                    )
                }
            } else {
                // println!("Sand will fall into the abyss to the right.");
                Point {
                    x: sand.x + 1,
                    y: bottom - 1,
                }
            }
        } else {
            // println!("Testing whether sand can fall left.");
            drop_sand_with_bottom(
                cave,
                Point {
                    x: sand.x - 1,
                    y: *y,
                },
                bottom,
            )
        }
    } else {
        // println!("Sand will fall into the abyss to the left.");
        Point {
            x: sand.x - 1,
            y: bottom - 1,
        }
    }
}

fn problem_1(cave: &Cave) {
    let mut cave = cave.to_owned();
    let mut count = 0;

    while let Some(Point { x, y }) = drop_sand(&mut cave, START) {
        // println!("{} -> ({}, {})", count, x, y);
        cave.get_mut(&x).unwrap().insert(y, Elem::Sand);

        // // Draw cave...
        // for y in 0..=10 {
        //     for x in 493..=504 {
        //         print!(
        //             "{}",
        //             match cave.get(&x).and_then(|vert| vert.get(&y)) {
        //                 Some(Elem::Rock) => "#",
        //                 Some(Elem::Sand) => "o",
        //                 None => " ",
        //             }
        //         );
        //     }
        //     println!();
        // }

        count += 1;
    }

    println!("Problem 1: {}", count);
}

fn problem_2(cave: &Cave) {
    let mut cave = cave.to_owned();
    let mut count = 0;
    let bottom = *cave.values().flat_map(|vert| vert.keys()).max().unwrap() + 2;

    loop {
        let pt = drop_sand_with_bottom(&mut cave, START, bottom);
        println!("{} -> ({}, {})", count, pt.x, pt.y);
        cave.entry(pt.x).or_default().insert(pt.y, Elem::Sand);

        // // Draw cave...
        // for y in 0..=12 {
        //     for x in 493..=504 {
        //         print!(
        //             "{}",
        //             match cave.get(&x).and_then(|vert| vert.get(&y)) {
        //                 Some(Elem::Rock) => "#",
        //                 Some(Elem::Sand) => "o",
        //                 None => " ",
        //             }
        //         );
        //     }
        //     println!();
        // }

        count += 1;
        if pt == START {
            break;
        }
    }

    println!("Problem 1: {}", count);
}

fn main() -> anyhow::Result<()> {
    // let cave = parse_input("example.txt")?;
    let cave = parse_input("input.txt")?;
    // println!("{:?}", cave);

    problem_1(&cave);
    problem_2(&cave);

    Ok(())
}
