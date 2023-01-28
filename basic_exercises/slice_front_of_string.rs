use std::io;

fn main() {
    println!("Enter string to slice: ");
    let input = get_input();
    println!("Enter num: ");
    let num = parse(get_input());
    println!("Original Word: {input}");
    let new_word = remove_chars(input, num);
    println!("Edited Word: {new_word}");
}

fn remove_chars(val: String, num: usize) -> String {
    let new_val = String::from(val);
    let slice = &new_val[num..];
    slice.to_string()
}

fn parse(val: String) -> usize {
    let err = "Cannot parse input.";
    val.trim().parse().expect(err)
}

fn get_input() -> String {
    let mut val = String::new();
    let err = "Cannot read input.";
    io::stdin().read_line(&mut val).expect(err);
    val
}
