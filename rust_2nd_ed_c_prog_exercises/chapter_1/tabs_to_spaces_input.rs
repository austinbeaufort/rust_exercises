// replace input tabs with four spaces
use std::io;

fn main() {
    let input = get_input();
    let result = replace_tabs(input);
    println!("{result}");
}

fn replace_tabs(input: String) -> String {
    let mut new_str = String::new();
    for ch in input.chars() {
        if ch == '\t' {
            new_str.push_str("    ");
        }
        new_str.push_str(ch.to_string().as_str());
    }
    new_str
}

fn get_input() -> String {
    let mut val = String::new();
    let err = "cannot read input string";
    io::stdin().read_line(&mut val).expect(err);
    val
}
