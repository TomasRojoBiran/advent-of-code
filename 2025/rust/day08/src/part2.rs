use std::collections::BinaryHeap;

#[derive(Debug)]
struct DisjointSet {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DisjointSet {
    fn find(&mut self, point: usize) -> usize {
        if self.parent[point] != point {
            self.parent[point] = self.find(self.parent[point]);
            self.parent[point]
        } else {
            point
        }
    }

    fn union(&mut self, root_a: usize, root_b: usize) {
        if root_a != root_b {
            if self.size[root_a] >= self.size[root_b] {
                self.parent[root_b] = root_a;
                self.size[root_a] += self.size[root_b]
            } else {
                self.parent[root_a] = root_b;
                self.size[root_b] += self.size[root_a]
            }
        }
    }
}

fn squared_dist(p: [i64; 3], q: [i64; 3]) -> i64 {
    (p[0] - q[0]).pow(2) + (p[1] - q[1]).pow(2) + (p[2] - q[2]).pow(2)
}

fn kruskal(points: &[[i64; 3]]) -> (usize, usize) {
    let mut edges = BinaryHeap::new();

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let d = squared_dist(points[i], points[j]);
            edges.push((d, i, j));
        }
    }
    let edges = edges.into_sorted_vec();

    let mut disjoint_set_points = DisjointSet {
        parent: (0..points.len()).collect(),
        size: vec![1; points.len()],
    };

    let mut last_edge = (0, 0);
    for (_, i, j) in edges {
        let root_i = disjoint_set_points.find(i);
        let root_j = disjoint_set_points.find(j);

        if root_i != root_j {
            disjoint_set_points.union(root_i, root_j);
            last_edge = (i, j);
        }
    }

    last_edge
}

pub fn solve(parsed: Vec<[i64; 3]>) -> i64 {
    let last_edge = kruskal(&parsed);
    parsed[last_edge.0][0] * parsed[last_edge.1][0]
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

        assert_eq!(result, 25272);
        Ok(())
    }
}
