use std::ops::Range;

use crate::parse2::Op;

pub fn solve(parsed: (Vec<Op>, Vec<String>)) -> u64 {
    let (ops, nums) = parsed;

    let mut problems: Vec<Range<usize>> = Vec::new();
    let mut start_col = 0;

    (0..nums[0].len()).for_each(|col| {
        if nums.iter().all(|row| row.as_bytes()[col] == b' ') {
            problems.push(start_col..col);
            start_col = col + 1
        }
    });
    if start_col < nums[0].len() {
        problems.push(start_col..nums[0].len());
    }

    ops.iter()
        .zip(problems)
        .map(|(op, range)| {
            let vals: Vec<u64> = range
                .map(|col| {
                    nums.iter()
                        .map(|row| row.as_bytes()[col] as char)
                        .filter(|ch| *ch != ' ')
                        .collect::<String>()
                        .parse()
                        .unwrap()
                })
                .collect();

            match op {
                Op::Add => vals.iter().sum::<u64>(),
                Op::Mul => vals.iter().product::<u64>(),
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse2::parse;

    #[test]
    fn solves_sample_input() -> Result<(), Box<dyn std::error::Error>> {
        let input = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";
        let parsed = parse(input)?;
        let result = solve(parsed);

        assert_eq!(result, 3263827);
        Ok(())
    }
}
