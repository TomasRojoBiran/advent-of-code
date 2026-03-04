pub fn solve(parsed: Vec<Vec<char>>) -> u64 {
    let mut ways = vec![vec![0; parsed[0].len()]; parsed.len()];

    let base_timeline: (usize, usize) = (
        1,
        parsed
            .first()
            .unwrap()
            .iter()
            .position(|ch| *ch == 'S')
            .unwrap(),
    );
    let (base_row, base_col) = base_timeline;
    ways[base_row][base_col] = 1;

    for row in 0..parsed.len() {
        for col in 0..parsed[0].len() {
            if parsed.get(row + 1).is_some() {
                if parsed[row + 1][col] != '^' {
                    ways[row + 1][col] += ways[row][col]
                } else if parsed[row + 1][col] == '^' {
                    ways[row + 1][col - 1] += ways[row][col];
                    ways[row + 1][col + 1] += ways[row][col]
                }
            }
        }
    }

    ways.last().unwrap().iter().sum()
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

        assert_eq!(result, 40);
        Ok(())
    }
}
