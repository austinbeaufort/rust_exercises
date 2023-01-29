use std::io;

struct Counts {
    blanks: i32,
    tabs: i32,
    new_lines: i32,
}


fn main() {
    let val = get_input();
    let counts = get_counts(val);
    println!("blanks: {}, tabs: {}, new_lines {}",
             counts.blanks, counts.tabs, counts.new_lines);
}

fn get_counts(val: String) -> Counts {
    let mut counts = Counts {
        blanks: 0,
        tabs: 0,
        new_lines: 0,
    };

    for char in val.chars() {
        match char {
            ' ' => counts.blanks += 1,
            '\t' => counts.tabs += 1,
            '\n' => counts.new_lines += 1,
            _ => continue
        }
    }
    counts

}


fn get_input() -> String {
    let mut val = String::new();
    let err = "cannot read input";
    io::stdin().read_line(&mut val).expect(err);
    val
}
