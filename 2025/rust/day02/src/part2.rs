use std::ops::RangeInclusive;

pub fn solve(parsed: Vec<RangeInclusive<u64>>) -> Result<u64, Box<dyn std::error::Error>> {
    let mut total = 0;
    for ids in parsed.into_iter() {
        for id in ids.into_iter() {
            let id_str = id.to_string();
            let half = id_str.len() / 2;
            for limit in 0..half {
                if id_str.len().rem_euclid(limit + 1) == 0 {
                    let all_match = id_str[0..=limit]
                        .chars()
                        .cycle()
                        .zip(id_str.chars())
                        .all(|(a, b)| a == b);
                    if all_match {
                        total += id;
                        break;
                    }
                }
            }
        }
    }
    Ok(total)
}
