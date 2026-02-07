use day01::parse::parse;
use day01::part1::solve;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../../input.txt");
    let parsed = parse(input);
    let result = solve(parsed)?;
    println!("{}", result);
    Ok(())
}
