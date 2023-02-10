// reverse line of user input without using built in rev() function
use std::io;

fn main() {
    println!("========== STRING AND WORD REVERSER ==========");
    println!("enter input to reverse: ");
    let input = get_input();
    let no_newline_input = remove_newline(input);
    let input_vec: Vec<&str> = no_newline_input.split(' ').collect();
    let reversed_string = reverse_words(reverse_vec(input_vec)).join(" ");
    println!("\nREVERSED INPUT: ");
    println!("{reversed_string}");
}

fn reverse_words(vec: Vec<&str>) -> Vec<String> {
    vec.into_iter().map(reverse_word).collect::<Vec<String>>()
}

fn reverse_word(word: &str) -> String {
    let mut new_word = String::new();
    for num in (0..word.len()).rev() {
        let char_str = get_char_as_str(num, word);
        new_word.push_str(&char_str);
    }
    new_word
}

fn get_char_as_str(num: usize, word: &str) -> String {
    // using unwrap() here because the program should panic if nth index doesn't exist
    word.chars().nth(num).unwrap().to_string()
}

fn reverse_vec(input_vec: Vec<&str>) -> Vec<&str> {
    let mut new_vec = Vec::new();
    for i in (0..input_vec.len()).rev() {
        new_vec.push(input_vec[i]);
    }
    new_vec
}

fn remove_newline(mut input: String) -> String {
    if input.ends_with('\n') {
        input.pop();
    }
    input
}

fn get_input() -> String {
    let mut val = String::new();
    let err = "cannot read input string";
    io::stdin().read_line(&mut val).expect(err);
    val
}
