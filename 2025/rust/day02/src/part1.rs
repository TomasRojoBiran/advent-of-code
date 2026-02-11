use std::ops::RangeInclusive;

pub fn solve(parsed: Vec<RangeInclusive<u64>>) -> Result<u64, Box<dyn std::error::Error>> {
    let total: u64 = parsed.into_iter().flat_map(|ids| {
        ids.into_iter().filter(|id| {
            let id_str = id.to_string();
            let half = id_str.len() / 2;
            &id_str[..half] == &id_str[half..]
        })
    }).sum();

    Ok(total)
}
