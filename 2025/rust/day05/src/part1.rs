use std::ops::RangeInclusive;

pub fn solve(parsed: (Vec<RangeInclusive<u64>>, Vec<u64>)) -> u64 {
    let (mut id_ranges, available_ids) = parsed;
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

    available_ids.into_iter().fold(0, |mut acc, id| {
        let mut right = merged_id_ranges.len() - 1;
        let mut left = 0;
        loop {
            if left <= right {
                let mid = left + (right - left) / 2;
                if id >= *merged_id_ranges[mid].start() && id <= *merged_id_ranges[mid].end() {
                    acc += 1
                } else if id < *merged_id_ranges[mid].start() {
                    if mid == 0 {
                        break acc;
                    }
                    right = mid - 1;
                    continue;
                } else {
                    left = mid + 1;
                    continue;
                }
            }
            break acc;
        }
    })
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

        assert_eq!(result, 3);
        Ok(())
    }
}
