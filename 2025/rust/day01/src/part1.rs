use crate::parse::Direction;

const STARTING_POSITION: i32 = 50;
const DIAL_SIZE: i32 = 100;

pub fn solve(parsed: Vec<Direction>) -> Result<i32, Box<dyn std::error::Error>> {
    let (_, final_count) = parsed.into_iter().fold(
        (STARTING_POSITION, 0),
        |(position, count), rot| {
            let delta = match rot {
                Direction::Left(n) => -n,
                Direction::Right(n) => n,
            };

            let new_position = (position + delta).rem_euclid(DIAL_SIZE);
            let new_count = if new_position == 0 {
                count + 1
            } else {
                count
            };

            (new_position, new_count)
        },
    );

    Ok(final_count)
}
