use day03::part2::solve;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../../input.txt");
    let result = solve(input);
    println!("{}", result);
    Ok(())
}

