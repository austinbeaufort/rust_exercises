// Problem description at bottom of file
use std::io;

enum Bracket {
    B1,
    B2,
    B3,
}

struct Costs {
    b1: f32,
    b2: f32,
    b3: f32,
}

fn main() {
    let num = parse(get_input());
    let bracket = get_bracket(num);
    let tax = get_tax(bracket, num);
    println!("Your tax total is: {tax}");
}

fn get_tax(bracket: Bracket, num: f32) -> f32 {
    let costs = get_costs(num);
    match bracket {
        Bracket::B3 => costs.b3,
        Bracket::B2 => costs.b2,
        Bracket::B1 => costs.b1,
    }
}

fn get_costs(num: f32) -> Costs {
    let b1 = 0.0;
    let b2 = (num - 10_000.0) * 0.1;
    let b3 = ((num - 20_000.0) * 0.2) + (10_000.0 * 0.1);
    Costs { b1, b2, b3 }
}

fn get_bracket(num: f32) -> Bracket {
    if num >= 20000.0 {
        Bracket::B3
    } else if num >= 10000.0 {
        Bracket::B2
    } else {
        Bracket::B1
    }
}

fn parse(val: String) -> f32 {
    let err = "Unable to parse input";
    val.trim().parse().expect(err)
}

fn get_input() -> String {
    let mut val = String::new();
    let err = "Unable to read input";
    println!("Enter a value: ");
    io::stdin().read_line(&mut val).expect(err);
    val
}

//Calculate income tax for the given income by
//adhering to the below rules:

// Taxable Income	Rate (in %)
// First $10,000	0
// Next $10,000	10
// The remaining	20
