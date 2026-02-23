use std::ops::RangeInclusive;

pub fn solve(parsed: (Vec<RangeInclusive<u64>>, Vec<u64>)) -> u64 {
    let (mut id_ranges, _) = parsed;
    id_ranges.sort_by_key(|r| (*r.start(), *r.end()));
    let mut current_range: Option<RangeInclusive<u64>> = None;
    let mut merged_id_ranges: Vec<RangeInclusive<u64>> = vec![];

    for r in &id_ranges {
        match current_range {
            None => current_range = Some(*r.start()..=*r.end()),
            Some(ref curr) => {
                if *r.start() <= *curr.end() + 1 {
                    let new_end = *curr.end().max(r.end());
                    current_range = Some(*curr.start()..=new_end)
                } else {
                    merged_id_ranges.push(current_range.take().unwrap());
                    current_range = Some(*r.start()..=*r.end())
                }
            }
        }
    }

    if let Some(curr) = current_range {
        merged_id_ranges.push(curr)
    }

    merged_id_ranges
        .into_iter()
        .fold(0, |acc, range| acc + *range.end() - *range.start() + 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::parse;

    #[test]
    fn solves_sample_input() -> Result<(), Box<dyn std::error::Error>> {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        let parsed = parse(input)?;
        let result = solve(parsed);

        assert_eq!(result, 14);
        Ok(())
    }
}
