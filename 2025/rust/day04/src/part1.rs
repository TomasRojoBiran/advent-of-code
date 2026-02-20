const NEIGHBORS: [(i32, i32); 8] = [
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
    (-1, -1),
    (-1, 1),
    (1, -1),
    (1, 1)
];

pub fn solve(parsed: Vec<Vec<char>>) -> u64 {
    let mut count = 0;
    let mut count_matrix = vec![vec![0; parsed[0].len()]; parsed.len()];
    for i in 0..parsed.len() {
        for j in 0..parsed[i].len() {
            if parsed[i][j] == '@' {
                for (di, dj) in NEIGHBORS {
                    let ni = i as i32 + di;
                    let nj = j as i32 + dj;
                    if (0..parsed.len() as i32).contains(&ni)
                        && (0..parsed[ni as usize].len()).contains(&(nj as usize))
                        && parsed[ni as usize][nj as usize] == '@'
                    {
                        count_matrix[i][j] += 1
                    }
                }
                if count_matrix[i][j] < 4 {
                    count += 1
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::parse;

    #[test]
    fn solves_sample_input() -> Result<(), Box<dyn std::error::Error>> {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let parsed = parse(input);
        let result = solve(parsed);

        assert_eq!(result, 13);
        Ok(())
    }
}
