use std::ops::RangeInclusive;

#[derive(Debug)]
enum Parity {
    Even(u64),
    Odd(u64),
}

pub fn solve(parsed: Vec<RangeInclusive<u64>>) -> Result<u64, Box<dyn std::error::Error>> {
    println!("{:?}", parsed);
    todo!("Implement Solve!");
}
