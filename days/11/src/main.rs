use anyhow::Result;

mod input;

use input::Monkey;

fn problem_1(monkeys: &[Monkey]) {
    const ROUNDS: usize = 20;

    let mut monkeys: Vec<_> = monkeys.to_vec();
    let mut passes: Vec<_> = std::iter::repeat(0).take(monkeys.len()).collect();

    for _round in 0..ROUNDS {
        for m_idx in 0..monkeys.len() {
            let items = std::mem::take(&mut monkeys[m_idx].items);

            for item in items {
                passes[m_idx] += 1;
                let worry_level = (monkeys[m_idx].operation)(item) / 3;
                let target_idx = match worry_level % monkeys[m_idx].modulo_test == 0 {
                    true => monkeys[m_idx].if_true,
                    false => monkeys[m_idx].if_false,
                };
                monkeys[target_idx].items.push(worry_level);
            }
        }
    }

    passes.sort();
    println!("{:?}", passes);

    let score: i64 = passes.iter().rev().take(2).product();
    println!("Score: {}", score);

}

fn problem_2(monkeys: &[Monkey]) {
    const ROUNDS: usize = 10000;

    let mut monkeys: Vec<_> = monkeys.to_vec();
    let mut passes: Vec<_> = std::iter::repeat(0).take(monkeys.len()).collect();

    let lcm: i64 = monkeys.iter().map(|m| m.modulo_test).product();

    for _round in 0..ROUNDS {
        for m_idx in 0..monkeys.len() {
            let items = std::mem::take(&mut monkeys[m_idx].items);

            for item in items {
                passes[m_idx] += 1;
                let worry_level = (monkeys[m_idx].operation)(item) % lcm;
                let target_idx = match worry_level % monkeys[m_idx].modulo_test == 0 {
                    true => monkeys[m_idx].if_true,
                    false => monkeys[m_idx].if_false,
                };
                monkeys[target_idx].items.push(worry_level);
            }
        }
    }

    passes.sort();
    println!("{:?}", passes);

    let score: i64 = passes.iter().rev().take(2).product();
    println!("Score: {}", score);

}
fn main() -> Result<()> {
    let monkeys = input::get_input();
    problem_1(&monkeys);
    problem_2(&monkeys);

    Ok(())
}
