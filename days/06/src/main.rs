use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use anyhow::Result;

fn main() -> Result<()> {
    // let stream = parse_input("./example.txt")?;
    let stream = parse_input("./input.txt")?;

    let package_start = find_start(&stream[..], 4);
    println!("Problem 1: {:?}", package_start);
    let message_start = find_start(&stream[..], 14);
    println!("Problem 2: {:?}", message_start);

    Ok(())
}

fn parse_input(path: &str) -> Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut stream = Vec::new();
    let bytes_read = file.read_to_end(&mut stream)?;
    println!("Read {} bytes from {}", bytes_read, path);

    Ok(stream)
}

fn find_start(stream: &[u8], chunk_length: usize) -> Option<usize> {
    let mut res: Option<usize> = None;
    let mut idx: usize = 0;

    // Create a map to quickly check if the chunk already contains the value. The value of the map
    // is the offset of the byte.
    let mut map: HashMap<&u8, usize> = HashMap::with_capacity(chunk_length);

    'outer: loop {
        if idx + chunk_length >= stream.len() {
            break;
        }

        // Get a reference to the next 4 chunks of the stream.
        let chunk = &stream[idx..idx + chunk_length];

        // Loop through each byte and add it to the set. As soon as we get to a duplicate element,
        // we can increment the index by at least that much and continue to the next iteration
        // immediately.
        map.clear();
        for (i, byte) in chunk.iter().enumerate() {
            // println!("set: {:?}, adding: {} offset: {}", map, byte, i);
            if !map.contains_key(byte) {
                map.insert(byte, i);
            } else {
                // println!("Found duplicate, jumping forward by {}", i);
                idx += map.get(byte).unwrap() + 1;
                continue 'outer;
            }
        }

        // If we get here, then chunks must contain n unique bytes.
        res = Some(idx + chunk_length);
        break;
    }

    res
}
