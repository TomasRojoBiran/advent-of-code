use regex::Regex;

pub fn parse(input: &str) -> Vec<(Vec<&str>, Vec<Vec<u8>>, Vec<u32>)> {
    // TODO: learn how to write regex, and parse this input with Regex to test that.
    let re = Regex::new(r"(?m)^\[(.*?)\]\s\((.*?)\)\s\{(.*?)\}$").unwrap();
    println!("{:?}", input.lines());
    let god: Vec<_> = re.captures_iter(input).collect();
    println!("{:?}", god);
    todo!()
}
