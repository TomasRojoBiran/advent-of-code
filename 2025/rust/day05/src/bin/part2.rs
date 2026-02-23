use day05::parse::parse;
use day05::part2::solve;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../../input.txt");
    let parsed = parse(input)?;
    let result = solve(parsed);
    println!("{}", result);
    Ok(())
}
