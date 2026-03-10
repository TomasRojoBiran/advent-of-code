#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::parse;

    #[test]
    fn solves_sample_input() -> Result<(), Box<dyn std::error::Error>> {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        let parsed = parse(input);
        let result = solve(parsed);

        assert_eq!(result, 50);
        Ok(())
    }
}
