use std::io;

fn main() {
    // without .len() method
    let val = get_input();
    let count = count_chars(&val);
    println!("{count}");
    // using .len() method
    println!("{}", val.len());
}

fn count_chars(val: &str) -> i32 {
    let mut count = 0;
    for _ in val.chars() {
        count += 1;
    }
    count
}

fn get_input() -> String {
    let mut val = String::new();
    let err = "cannot read input";
    io::stdin().read_line(&mut val).expect(err);
    val
}

