use std::fs;

struct Counts {
    chars: i32,
    words: i32,
    new_lines: i32,
}


fn main() {
    let contents = get_file_contents();
    let counts = get_counts(&contents);
    println!("char count: {}", counts.chars);
    println!("word count: {}", counts.words);
    println!("new line count: {}", counts.new_lines);
}

fn get_counts(contents: &str) -> Counts {
    let mut in_word = false;
    let mut counts = Counts {
        chars: 0,
        words: 0,
        new_lines: 0,
    };

    for char in contents.chars() {
        counts.chars += 1;

        if is_blank(char) && char == '\n' {
            counts.new_lines += 1;
            in_word = false;
        } else if is_blank(char) {
            in_word = false;
        } else if in_word == false {
            counts.words += 1;
            in_word = true;
        }
    }
    counts
}


fn is_blank(ch: char) -> bool {
    ch == '\n' || ch == '\t' || ch == ' '
}

fn get_file_contents() -> String {
    let file_path = "src/my_file.txt";
    let err = "Could not read contents from file.";
    fs::read_to_string(file_path).expect(err)
}
