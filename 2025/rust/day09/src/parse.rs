pub fn parse(input: &str) -> Result<Vec<(u64, u64)>, Box<dyn std::error::Error>> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').ok_or("missing coma")?;
            Ok((x.parse()?, y.parse()?))
        })
        .collect()
}
