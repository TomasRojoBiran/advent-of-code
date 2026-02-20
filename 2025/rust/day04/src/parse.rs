pub fn parse(input: &str) -> Vec<Vec<&str>> {
    input
        .lines()
        .map(|line| vec![line])
        .collect()
}
