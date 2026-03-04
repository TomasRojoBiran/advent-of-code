use std::collections::HashSet;

pub fn solve(parsed: Vec<Vec<char>>) -> u64 {
    let mut beams_seen = HashSet::new();
    let mut beams_queue = Vec::new();

    let base_beam: (usize, usize) = (
        1,
        parsed
            .first()
            .unwrap()
            .iter()
            .position(|ch| *ch == 'S')
            .unwrap(),
    );
    beams_queue.push(base_beam);

    let mut splits = 0;
    while let Some((row, col)) = beams_queue.pop() {
        if parsed.get(row + 1).is_some() {
            if parsed[row + 1][col] != '^' && !beams_seen.contains(&(row + 1, col)) {
                beams_queue.push((row + 1, col));
                beams_seen.insert((row + 1, col));
            } else if parsed[row + 1][col] == '^' {
                beams_queue.push((row + 1, col - 1));
                beams_queue.push((row + 1, col + 1));
                splits += 1;
            }
        }
    }

    splits
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::parse;

    #[test]
    fn solves_sample_input() -> Result<(), Box<dyn std::error::Error>> {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        let parsed = parse(input);
        let result = solve(parsed);

        assert_eq!(result, 21);
        Ok(())
    }
}
