use ndarray::Array2;

#[derive(Debug)]
pub enum Op {
    Add,
    Mul,
}

pub fn parse(input: &str) -> Result<(Vec<Op>, Array2<u64>), &'static str> {
    let input_lines: Vec<&str> = input.lines().collect();
    let (op_line, num_lines) = input_lines.split_last().unwrap();

    let ops = op_line
        .split_whitespace()
        .map(|op| match op {
            "+" => Ok(Op::Add),
            "*" => Ok(Op::Mul),
            _ => Err("invalid op"),
        })
        .collect::<Result<Vec<Op>, _>>()?;

    let rows = num_lines.len();
    let cols = num_lines[0].split_whitespace().count();
    let nums_vec = num_lines
        .iter()
        .flat_map(|line| line.split_whitespace())
        .map(|num: &str| num.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let nums: Array2<u64> = Array2::from_shape_vec((rows, cols), nums_vec).unwrap();

    Ok((ops, nums))
}
