fn area(tile_a: &(u64, u64), tile_b: &(u64, u64)) -> u64 {
    (tile_a.0.abs_diff(tile_b.0) + 1) * (tile_a.1.abs_diff(tile_b.1) + 1)
}

pub fn solve(parsed: Vec<(u64, u64)>) -> u64 {
    let mut base = area(&parsed[0], &parsed[1]);
    for (i, tile_a) in parsed.iter().enumerate() {
        for tile_b in parsed.iter().skip(i + 1) {
            if area(tile_a, tile_b) > base {
                base = area(tile_a, tile_b)
            }
        }
    }
    base
}

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
        let parsed = parse(input)?;
        let result = solve(parsed);

        assert_eq!(result, 50);
        Ok(())
    }
}
