#[derive(Debug)]
pub enum Op {
    Add,
    Mul,
}

pub fn parse(input: &str) -> Result<(Vec<Op>, Vec<String>), &'static str> {
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

    let max_width = num_lines.iter().map(|row| row.len()).max().unwrap();
    let nums = num_lines
        .iter()
        .map(|row| {
            let missing = max_width - row.len();
            let mut str_row = row.to_string();
            str_row.push_str(&" ".repeat(missing));
            str_row
        })
        .collect::<Vec<String>>();

    Ok((ops, nums))
}
