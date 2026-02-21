pub fn solve(parsed: Vec<Vec<char>>) -> u64 {
    let mut count = 0;
    let mut count_matrix = vec![vec![0; parsed[0].len()]; parsed.len()];
    for i in 0..parsed.len() {
        for j in 0..parsed[i].len() {
            if parsed[i][j] == '@' {
                if let Some(si) = i.checked_sub(1) {
                    if let Some(sj) = j.checked_sub(1) {
                        if parsed[si][sj] == '@' {
                            count_matrix[i][j] += 1
                        }
                    }
                }
                if let Some(ai) = i.checked_add(1) {
                    if let Some(aj) = j.checked_add(1) {
                        if ai < parsed.len() && aj < parsed[ai].len() {
                            if parsed[ai][aj] == '@' {
                                count_matrix[i][j] += 1
                            }
                        }
                    }
                }
                if let Some(ai) = i.checked_add(1) {
                    if let Some(sj) = j.checked_sub(1) {
                        if ai < parsed.len() {
                            if parsed[ai][sj] == '@' {
                                count_matrix[i][j] += 1
                            }
                        }
                    }
                }
                if let Some(si) = i.checked_sub(1) {
                    if let Some(aj) = j.checked_add(1) {
                        if aj < parsed[si].len() {
                            if parsed[si][aj] == '@' {
                                count_matrix[i][j] += 1
                            }
                        }
                    }
                }
                if let Some(si) = i.checked_sub(1) {
                    if parsed[si][j] == '@' {
                        count_matrix[i][j] += 1
                    }
                }
                if let Some(ai) = i.checked_add(1) {
                    if ai < parsed.len() {
                        if parsed[ai][j] == '@' {
                            count_matrix[i][j] += 1
                        }
                    }
                }
                if let Some(sj) = j.checked_sub(1) {
                    if parsed[i][sj] == '@' {
                        count_matrix[i][j] += 1
                    }
                }
                if let Some(aj) = j.checked_add(1) {
                    if aj < parsed[i].len() {
                        if parsed[i][aj] == '@' {
                            count_matrix[i][j] += 1
                        }
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
