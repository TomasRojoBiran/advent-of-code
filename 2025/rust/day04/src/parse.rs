pub fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| {
            line
                .chars()
                .collect()
        })
        .collect()
}
