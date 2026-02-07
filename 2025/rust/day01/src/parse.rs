#[derive(Debug)]
pub enum Direction {
    Left(i32),
    Right(i32),
}

pub fn parse(input: &str) -> Vec<Direction> {
    input
        .lines()
        .map(|line| {
            let (dir_char, num_str) = line.split_at(1);
            let steps: i32 = num_str.parse().unwrap();

            match dir_char {
                "L" => Direction::Left(steps),
                "R" => Direction::Right(steps),
                _ => panic!("Invalid Direction: {}", dir_char),
            }
        })
        .collect()
}
