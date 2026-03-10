use itertools::Itertools;

fn area(tile_a: &(u64, u64), tile_b: &(u64, u64)) -> u64 {
    (tile_a.0.abs_diff(tile_b.0) + 1) * (tile_a.1.abs_diff(tile_b.1) + 1)
}

pub fn solve(parsed: Vec<(u64, u64)>) -> u64 {
    type Edge<'a> = (&'a (u64, u64), &'a (u64, u64));
    let edges: Vec<Edge> = parsed.iter().circular_tuple_windows().collect();
    parsed
        .iter()
        .tuple_combinations()
        .map(|(a, b)| (a, b, area(a, b)))
        .sorted_by_key(|v| v.2)
        .rev()
        .find(|(a, b, _)| {
            edges.iter().all(|(line_start, line_end)| {
                let rect_left = a.0.max(b.0) <= line_start.0.min(line_end.0);
                let rect_right = a.0.min(b.0) >= line_start.0.max(line_end.0);
                let rect_above = a.1.max(b.1) <= line_start.1.min(line_end.1);
                let rect_below = a.1.min(b.1) >= line_start.1.max(line_end.1);
                rect_left || rect_right || rect_above || rect_below
            })
        })
        .unwrap()
        .2
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

        assert_eq!(result, 24);
        Ok(())
    }
}
