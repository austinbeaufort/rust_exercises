// SPLIT INPUT INTO INDIVIDUAL WORDS AND PRINT EACH WORD ON A NEWLINE
use std::io;

fn main() {
    let input = get_input();
    let words = input.split_whitespace();
    println!("============== WORDS =============");
    words.for_each(|word| println!("{word}"));
}


fn get_input() -> String {
    let mut val = String::new();
    let err = "cannot read input";
    io::stdin().read_line(&mut val).expect(err);
    val
}
