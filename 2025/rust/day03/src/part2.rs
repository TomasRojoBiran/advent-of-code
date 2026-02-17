use itertools::Itertools;

pub fn solve(input: &str) -> u64 {
    input
        .lines()
        .map(|bank| {
            let mut batteries: Vec<char> = vec![];
            let mut current_index = 0;

            for i in 0..12 {
                let (index, first_max) = &bank
                    [current_index..(bank.len() - (12 - i) + 1)]
                    .chars()
                    .enumerate()
                    .max_set_by_key(|(_index, battery)| *battery)
                    .first()
                    .cloned()
                    .unwrap();

                batteries.push(*first_max);
                current_index += index + 1;
            }

            batteries
                .iter()
                .collect::<String>()
                .parse::<u64>()
                .unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_sample_input() -> Result<(), Box<dyn std::error::Error>> {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        let result = solve(input);

        assert_eq!(result, 3121910778619);
        Ok(())
    }
}
