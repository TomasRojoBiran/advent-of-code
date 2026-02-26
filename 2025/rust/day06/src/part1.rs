use crate::parse1::Op;
use ndarray::Array2;

pub fn solve(parsed: (Vec<Op>, Array2<u64>)) -> u64 {
    let (ops, nums) = parsed;

    ops.iter()
        .zip(nums.columns())
        .map(|(op, col)| match op {
            Op::Add => col.sum(),
            Op::Mul => col.product(),
        })
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse1::parse;

    #[test]
    fn solves_sample_input() -> Result<(), Box<dyn std::error::Error>> {
        let input = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";
        let parsed = parse(input)?;
        let result = solve(parsed);

        assert_eq!(result, 4277556);
        Ok(())
    }
}
