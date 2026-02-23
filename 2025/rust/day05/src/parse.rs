use std::ops::RangeInclusive;

pub fn parse(input: &str) -> Result<(Vec<RangeInclusive<u64>>, Vec<u64>), &'static str> {
    let (a, b) = input.split_once("\n\n").unwrap();
    let id_ranges: Vec<RangeInclusive<u64>> = a
        .lines()
        .map(|line| {
            let (start_str, end_str): (&str, &str) = line.split_once("-").unwrap();
            let start: u64 = start_str.parse().unwrap();
            let end: u64 = end_str.parse().unwrap();
            start..=end
        })
        .collect();
    let available_ids: Vec<u64> = b.lines().map(|line| line.parse().unwrap()).collect();

    Ok((id_ranges, available_ids))
}
