use std::ops::RangeInclusive;

pub fn parse(input: &str) -> Vec<RangeInclusive<u64>> {
    input
        .trim()
        .split(',')
        .map(|chunk| {
            let (start_str, end_str): (&str, &str) = chunk.split_once('-').unwrap();
            let start: u64 = start_str.parse().unwrap();
            let end: u64 = end_str.parse().unwrap();
            start..=end
        })
        .collect()
}
