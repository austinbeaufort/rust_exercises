use std::io;

fn main() {
    // without .len() method
    let val = get_input();
    let count = count_new_lines(val);
    // cunt will always be one currently because
    // read_line only returns 1 new line char
    println!("{count}");
}

fn count_new_lines(val: String) -> i32 {
    let mut count = 0;
    for char in val.chars() {
        println!("{char}");
        if char == '\n' {
            count += 1;
        }
    }
    count
}

fn get_input() -> String {
    let mut val = String::new();
    let err = "cannot read input";
    io::stdin().read_line(&mut val).expect(err);
    val
}
