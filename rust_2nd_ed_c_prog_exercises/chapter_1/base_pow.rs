// BASE TO A POWER WITHOUT USING BUILT IN pow() method
// note: result in this example will panic if total causes overflow
use std::io;

fn main() {
    println!("Enter base: ");
    let base = parse(get_input());
    println!("Enter power: ");
    let power = parse(get_input());
    let result = get_result(base, power);
    println!("{base} to the power of {power} equals {result}");
}

fn get_result(base: i128, pow: i128) -> i128 {
    let mut total = 1;
    for _ in 0..pow {
        total *= base;
    }
    total
}

fn parse(val: String) -> i128 {
    let err = "could not parse input";
    val.trim().parse().expect(err)
}

fn get_input() -> String {
    let mut val = String::new();
    let err = "cannot read input";
    io::stdin().read_line(&mut val).expect(err);
    val
}
