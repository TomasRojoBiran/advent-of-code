pub fn parse(input: &str) -> Result<Vec<Vec<u8>>, &'static str> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).map(|d| d as u8).ok_or("non-digit"))
                .collect()
        })
        .collect()
}
