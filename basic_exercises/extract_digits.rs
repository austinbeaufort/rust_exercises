// EXTRACT VALUE
// For example, If the given val given is 7536, the output will be “6 3 5 7“, with a space separating the digits.

use std::io;

fn main() {
    println!("Enter string of digits to extract");
    let val = get_input();
    let extracted_val = get_extracted_val(val);
    println!("Extracted Value: {extracted_val}");
}

fn get_extracted_val(val: String) -> String {
    val.chars().rev().map(add_space).collect()
}

fn add_space(ch: char) -> String {
    ch.to_string() + " "
}

fn get_input() -> String {
    let mut val = String::new();
    let err = "Error reading input.";
    io::stdin().read_line(&mut val).expect(err);
    val
}
