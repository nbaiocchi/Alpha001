use clap::Parser;

#[derive(Parser, Debug)]
pub struct CalculatorOptions {
    //#[clap(short = 'c', long)]
    pub first_value: i32,
    pub symbol: char,
    pub second_value: i32,
}

pub fn calculator(param: CalculatorOptions) {

    match param.symbol {
        '+' => println!("result: {}\n", param.first_value + param.second_value),
        '*' => println!("result: {}\n", param.first_value * param.second_value),
        _ => println!("You can only use + or * symbol\n"),
    }
}