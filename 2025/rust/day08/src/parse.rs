pub fn parse(input: &str) -> Vec<[i64; 3]> {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|str_num| str_num.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
                .try_into()
                .unwrap()
        })
        .collect()
}
