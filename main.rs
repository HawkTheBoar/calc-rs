use std::io::stdin;
enum Operation {
    Addition,
    Substraction,
    Division,
    Multiplication,
}
struct Number {
    operation: Option<Operation>,
}
fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
}
fn input_parse(str: &String, numbers: &mut Vec<Number>) {
    let num_index = 0;
    for (i, c) in str.chars().enumerate() {
        let mut num: Number;
        num.operation = Some(match c {
            '+' => Operation::Addition,
            '-' => Operation::Substraction,
            '*' => Operation::Multiplication,
            '/' => Operation::Division,
            _ => {
                continue;
            }
        });
    }
}
