// PROBLEM DESCRIPTION AT BOTTOM OF FILE
fn main() {
    let words = ["fleece", "kayak", "cowbell", "test"];
    let doubles_list = get_doubles_list(words);
    print_results(words, doubles_list);
}

fn print_results(words: [&str; 4], doubles_list: Vec<bool>) {
    for i in 0..4 {
        let word = words[i];
        if doubles_list[i] {
            println!("{word} contains two of the same char in a row.");
        } else {
            println!("{word} does NOT contain two of the same char in row.");
        }
    }
}

fn get_doubles_list(words: [&str; 4]) -> Vec<bool> {
    words.iter().map(check_doubles).collect()
}

fn check_doubles(word: &&str) -> bool {
    let mut last_char = String::new();
    for ch in word.chars() {
        if ch.to_string() == last_char {
            return true;
        }
        last_char = ch.to_string();
    }
    false
}

// The goal of this challenge is to analyze a string to check
// if it contains two of the same letter in a row. For example,
// the string "hello" has l twice in a row, while the string "nono"
// does not have two identical letters in a row.
