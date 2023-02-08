use std::fs;

fn main() {
    let contents = get_file_contents();

    println!("============== DIGITS, BLANKS, AND CHARS COUNTER ================");
    print_num_counts(&contents);

    let (blank_count, other_chars_count) = get_blank_and_chars_counts(&contents);
    println!("\nThe total blank count for the file is {blank_count}.");
    println!("The total count for all non-blank and non-digit chars in the file is {other_chars_count}.");
}


fn get_blank_and_chars_counts(contents: &str) -> (i32, i32) {
    let mut blank_count = 0;
    let mut other_chars_count = 0;

    for ch in contents.chars() {
        if ch.is_whitespace() {
            blank_count += 1;
        } else if ch.is_ascii_digit() {
            continue;
        } else {
            other_chars_count += 1;
        }
    }
    (blank_count, other_chars_count)
}


fn print_num_counts(contents: &str) {
    for num in 0..10 {
        let count = contents.matches(&num.to_string()).count();
        println!("The number {num} appears {count} times in the file.");
    }
}


fn get_file_contents() -> String {
    let path = "src/my_file.txt";
    let err = "could not read file";
    fs::read_to_string(path).expect(err)
}
