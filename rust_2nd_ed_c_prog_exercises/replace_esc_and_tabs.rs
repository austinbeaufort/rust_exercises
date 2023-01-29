// Replace tabs with \t, and \ with \\
    use std::io;

fn main() {
    let sentence = get_input();
    let new_sentence = format(sentence);
    println!("{}", new_sentence);
}

fn format(val: String) -> String {
    val
        .chars()
        .map(format_char)
        .collect::<Vec<String>>()
        .join("")
}

fn format_char(ch: char) -> String {
    match ch {
        '\\' => String::from("\\\\"),
        '\t' => String::from("\\t"),
        _    => ch.to_string()
    }
}

fn get_input() -> String {
    let mut val = String::new();
    let err = "cannot read input";
    io::stdin().read_line(&mut val).expect(err);
    val
}

