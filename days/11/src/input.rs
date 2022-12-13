use std::{
    fs::File,
    io::{BufReader, Read},
};

use anyhow::Result;


#[derive(Debug, Clone)]
pub struct Monkey {
    /// List of items belonging to each monkey.
    pub items: Vec<i64>,

    /// How to transform the input items.
    pub operation: fn(i64) -> i64,

    /// Test checks for divisibility.
    pub modulo_test: i64,

    /// Index of monkey that will receive.
    pub if_true: usize,
    pub if_false: usize,
}

pub fn get_example() -> Vec<Monkey> {
    vec![
        // Monkey 0:
        //   Starting items: 79, 98
        //   Operation: new = old * 19
        //   Test: divisible by 23
        //     If true: throw to monkey 2
        //     If false: throw to monkey 3
        Monkey {
            items: vec![79, 98],
            operation: |x| x * 19,
            modulo_test: 23,
            if_true: 2,
            if_false: 3,
        },
        // Monkey 1:
        //   Starting items: 54, 65, 75, 74
        //   Operation: new = old + 6
        //   Test: divisible by 19
        //     If true: throw to monkey 2
        //     If false: throw to monkey 0
        Monkey {
            items: vec![54, 65, 75, 74],
            operation: |x| x + 6,
            modulo_test: 19,
            if_true: 2,
            if_false: 0,
        },
        // Monkey 2:
        //   Starting items: 79, 60, 97
        //   Operation: new = old * old
        //   Test: divisible by 13
        //     If true: throw to monkey 1
        //     If false: throw to monkey 3
        Monkey {
            items: vec![79, 60, 97],
            operation: |x| x * x,
            modulo_test: 13,
            if_true: 1,
            if_false: 3,
        },
        // Monkey 3:
        //   Starting items: 74
        //   Operation: new = old + 3
        //   Test: divisible by 17
        //     If true: throw to monkey 0
        //     If false: throw to monkey 1
        Monkey {
            items: vec![74],
            operation: |x| x + 3,
            modulo_test: 17,
            if_true: 0,
            if_false: 1,
        },
    ]
}

pub fn get_input() -> Vec<Monkey> {
    vec![
        // Monkey 0:
        //   Starting items: 91, 66
        //   Operation: new = old * 13
        //   Test: divisible by 19
        //     If true: throw to monkey 6
        //     If false: throw to monkey 2
        Monkey {
            items: vec![91, 66],
            operation: |x| x * 13,
            modulo_test: 19,
            if_true: 6,
            if_false: 2,
        },
        // Monkey 1:
        //   Starting items: 78, 97, 59
        //   Operation: new = old + 7
        //   Test: divisible by 5
        //     If true: throw to monkey 0
        //     If false: throw to monkey 3
        Monkey {
            items: vec![78, 97, 59],
            operation: |x| x + 7,
            modulo_test: 5,
            if_true: 0,
            if_false: 3,
        },
        // Monkey 2:
        //   Starting items: 57, 59, 97, 84, 72, 83, 56, 76
        //   Operation: new = old + 6
        //   Test: divisible by 11
        //     If true: throw to monkey 5
        //     If false: throw to monkey 7
        Monkey {
            items: vec![57, 59, 97, 84, 72, 83, 56, 76],
            operation: |x| x + 6,
            modulo_test: 11,
            if_true: 5,
            if_false: 7,
        },
        //
        // Monkey 3:
        //   Starting items: 81, 78, 70, 58, 84
        //   Operation: new = old + 5
        //   Test: divisible by 17
        //     If true: throw to monkey 6
        //     If false: throw to monkey 0
        Monkey {
            items: vec![81, 78, 70, 58, 84],
            operation: |x| x + 5,
            modulo_test: 17,
            if_true: 6,
            if_false: 0,
        },
        // Monkey 4:
        //   Starting items: 60
        //   Operation: new = old + 8
        //   Test: divisible by 7
        //     If true: throw to monkey 1
        //     If false: throw to monkey 3
        Monkey {
            items: vec![60],
            operation: |x| x + 8,
            modulo_test: 7,
            if_true: 1,
            if_false: 3,
        },
        // Monkey 5:
        //   Starting items: 57, 69, 63, 75, 62, 77, 72
        //   Operation: new = old * 5
        //   Test: divisible by 13
        //     If true: throw to monkey 7
        //     If false: throw to monkey 4
        Monkey {
            items: vec![57, 69, 63, 75, 62, 77, 72],
            operation: |x| x * 5,
            modulo_test: 13,
            if_true: 7,
            if_false: 4,
        },
        // Monkey 6:
        //   Starting items: 73, 66, 86, 79, 98, 87
        //   Operation: new = old * old
        //   Test: divisible by 3
        //     If true: throw to monkey 5
        //     If false: throw to monkey 2
        Monkey {
            items: vec![73, 66, 86, 79, 98, 87],
            operation: |x| x * x,
            modulo_test: 3,
            if_true: 5,
            if_false: 2,
        },
        // Monkey 7:
        //   Starting items: 95, 89, 63, 67
        //   Operation: new = old + 2
        //   Test: divisible by 2
        //     If true: throw to monkey 1
        //     If false: throw to monkey 4
        Monkey {
            items: vec![95, 89, 63, 67],
            operation: |x| x + 2,
            modulo_test: 2,
            if_true: 1,
            if_false: 4,
        },
    ]
}

#[allow(dead_code)]
fn parse_input(path: &str) -> Result<Vec<Monkey>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut input = String::new();
    let bytes_read = reader.read_to_string(&mut input)?;
    assert!(bytes_read > 0);

    Ok(Vec::new())
}
