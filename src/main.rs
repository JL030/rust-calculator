use std::env::{args, Args};
fn main() {
    let mut args: Args = args();
    
    let first_number_string: String = args.nth(1).unwrap();
    let first_number = first_number_string.parse::<f32>().unwrap();

    let operator: char = args.nth(0).unwrap().chars().next().unwrap();

    let second_number_string: String = args.nth(0).unwrap();
    let second_number = second_number_string.parse::<f32>().unwrap();

    println!("{}", operate(operator, first_number, second_number));
}

fn operate(operator: char, first_number: f32, second_number: f32) -> String {
    let result =  match operator {
        '+'             => first_number + second_number,
        '-'             => first_number - second_number,
        '/'             => first_number / second_number,
        '*' | 'X' | 'x' => first_number * second_number,
        _               => panic!("Operator is not defined")
    };

    return format(operator, first_number, second_number, result);
}

fn format(operator: char, first_number: f32, second_number: f32, result: f32) -> String {
    return format!(
        "{} {} {} = {}",
        first_number,
        operator,
        second_number,
        result
    );
}
