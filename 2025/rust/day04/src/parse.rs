pub fn parse(input: &str) -> Result<Vec<Vec<&str>>, &'static str> {
    let parsed = input
        .lines()
        .map(|line| vec![line])
        .collect();

    Ok(parsed)
}
