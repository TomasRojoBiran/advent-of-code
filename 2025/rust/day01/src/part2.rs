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

            let raw_new = position + delta;
            let new_position = (raw_new).rem_euclid(DIAL_SIZE);

            let hits = if delta > 0 {
                raw_new.div_euclid(DIAL_SIZE) - position.div_euclid(DIAL_SIZE)
            } else if delta < 0 {
                (position - 1).div_euclid(DIAL_SIZE) - (raw_new - 1).div_euclid(DIAL_SIZE)
            } else {
                0
            };

            let new_count = count + hits;

            (new_position, new_count)
        },
    );

    Ok(final_count)
}
