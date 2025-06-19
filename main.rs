use std::io::stdin;
enum Operation {
    Addition,
    Subtraction,
    Division,
    Multiplication,
}
struct Number {
    operation: Option<Operation>,
    value: i32,
}
fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
}
fn input_parse(input: &str) {
    let mut numbers: Vec<Number> = Vec::new();
    let mut num_index = 0;
    for (i, c) in input.chars().enumerate() {
        let op = Some(match c {
            '+' => Operation::Addition,
            '-' => Operation::Subtraction,
            '*' => Operation::Multiplication,
            '/' => Operation::Division,
            _ => {
                continue;
            }
        });

        // first argument has no operation
    }
    // add last argument
}
