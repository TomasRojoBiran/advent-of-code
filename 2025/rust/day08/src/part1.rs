use std::collections::BinaryHeap;

const K_CLOSEST: usize = 1000;

fn squared_dist(p: [i64; 3], q: [i64; 3]) -> i64 {
    (p[0] - q[0]).pow(2) + (p[1] - q[1]).pow(2) + (p[2] - q[2]).pow(2)
}

pub fn solve(parsed: Vec<[i64; 3]>) -> u64 {
    let mut edges = BinaryHeap::new();

    for i in 0..parsed.len() {
        for j in i + 1..parsed.len() {
            let d = squared_dist(parsed[i], parsed[j]);
            if edges.len() < K_CLOSEST {
                edges.push((d, i, j));
            } else if d < edges.peek().unwrap().0 {
                edges.pop();
                edges.push((d, i, j))
            }
        }
    }
    let edges = edges.into_sorted_vec();

    println!("{:?}", edges);
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::parse;

    #[test]
    fn solves_sample_input() -> Result<(), Box<dyn std::error::Error>> {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        let parsed = parse(input);
        let result = solve(parsed);

        assert_eq!(result, 40);
        Ok(())
    }
}
