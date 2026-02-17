use std::cmp::max;

pub fn solve(parsed: Vec<Vec<u8>>) -> u64 {
    parsed
        .into_iter()
        .map(|bank| {
            let mut best_for_bank = 0;
            for i in 0..bank.len() - 1 {
                let mut best_ones = 0;
                for j in i+1..bank.len() {
                    if bank[j] > best_ones {
                        best_ones = bank[j];
                    }
                }
                let candidate = (bank[i] * 10 + best_ones) as u64;
                best_for_bank = max(best_for_bank, candidate);
            }
            best_for_bank
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::parse;

    #[test]
    fn solves_sample_input() -> Result<(), Box<dyn std::error::Error>> {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        let parsed = parse(input)?;
        let result = solve(parsed);

        assert_eq!(result, 357);
        Ok(())
    }
}
