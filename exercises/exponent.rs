// return result from given base and exponent
// do not use the available exponent function
use std::io;

fn main() {
    println!("Enter a base value: ");
    let base = parse(get_input());
    println!("Enter an exponent value: ");
    let pow = parse(get_input());

    let long_form = get_long_form(base, pow);
    let result = get_total(base, pow);
    println!("{base} raised to the power of {pow}: {result}");
    println!("i.e ({long_form}) = {result}");
}

fn get_long_form(base: i32, pow: i32) -> String {
    let mut val = String::new();
    for x in 1..(pow + 1) {
        let current = build_string(base, pow, x);
        val.push_str(&current);
    }
    val
}

fn build_string(base: i32, pow: i32, x: i32) -> String {
    let mut current = base.to_string();
    if x != pow {
        current += " * ";
    }
    current
}

fn get_total(base: i32, pow: i32) -> i32 {
    let mut total = 1;
    for _ in 0..pow {
        total *= base;
    }
    total
}

fn parse(val: String) -> i32 {
    let err = "Unable to parse input.";
    val.trim().parse().expect(err)
}

fn get_input() -> String {
    let mut val = String::new();
    let err = "Unable to get input.";
    io::stdin().read_line(&mut val).expect(err);
    val
}

