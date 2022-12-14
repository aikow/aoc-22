use std::fs::File;
use std::io::{BufReader, Read};
use std::slice;
use std::str::FromStr;

use packet::Packet;

mod packet;

fn parse_input(path: &str) -> anyhow::Result<Vec<(Packet, Packet)>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut input = String::new();
    let _read = reader.read_to_string(&mut input)?;

    Ok(input
        .split("\n\n")
        .map(|pair| {
            let mut pairs = pair.split('\n');
            let fst = Packet::from_str(pairs.next().unwrap()).unwrap();
            let snd = Packet::from_str(pairs.next().unwrap()).unwrap();

            (fst, snd)
        })
        .collect())
}

fn problem_1(packets: &[(Packet, Packet)]) {
    let num_sorted: usize = packets
        .iter()
        .enumerate()
        .filter_map(|(i, (p1, p2))| if p1 < p2 { Some(i + 1) } else { None })
        .sum();
    println!("Number of sorted packets: {}", num_sorted);
}

fn problem_2(packets: &[(Packet, Packet)]) {
    let div1 = Packet::from_str("[[2]]").unwrap();
    let div2 = Packet::from_str("[[6]]").unwrap();

    let mut sorted: Vec<_> = packets.iter().flat_map(|(p1, p2)| [p1, p2]).collect();
    sorted.push(&div1);
    sorted.push(&div2);
    sorted.sort();

    let idx1 = sorted.binary_search(&&div1).unwrap() + 1;
    let idx2 = sorted.binary_search(&&div2).unwrap() + 1;

    println!("Multiplied indices of divider packets: {}", idx1 * idx2);
}

fn main() -> anyhow::Result<()> {
    // let packets = parse_input("example.txt")?;
    let packets = parse_input("input.txt")?;

    for (p1, p2) in &packets {
        println!("{}", p1);
        println!("{}", p2);
        println!();
    }

    problem_1(&packets);
    problem_2(&packets);

    Ok(())
}
