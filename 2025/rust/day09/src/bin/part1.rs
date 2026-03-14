use day09::parse::parse;
use day09::part1::solve;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../../input.txt");
    let parsed = parse(input)?;
    let result = solve(parsed);
    println!("{}", result);
    Ok(())
}
