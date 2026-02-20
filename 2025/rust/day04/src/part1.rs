pub fn solve(parsed: Vec<Vec<&str>>) -> u64 {
    println!("{:?}", parsed);
    todo!("Implement solve")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::parse;

    #[test]
    fn solves_sample_input() -> Result<(), Box<dyn std::error::Error>> {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let parsed = parse(input);
        let result = solve(parsed);

        assert_eq!(result, 13);
        Ok(())
    }
}
