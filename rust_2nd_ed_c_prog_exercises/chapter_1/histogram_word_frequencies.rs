// HORIZONTAL HISTOGRAM OF WORD FREQUENCIES IN A file
use std::fs;
use std::cmp::max;
use std::env;


fn main() {
    println!("================= HISTOGRAM CREATOR ==================");
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let hist_type = &args[2];
    let contents = get_file_contents(path);
    let words = contents.split_whitespace().map(String::from).collect();
    let word_lengths = format_words(words);
    let max_length = get_max(&word_lengths);
    print_histogram(&word_lengths, max_length, hist_type);
}

fn print_histogram(word_lengths: &Vec<usize>, max_length: usize, hist_type: &str) {
    if hist_type == "horizontal" {
        for num in 1..(max_length + 1) {
            let symbol = "|";
            let (hist_row, new_num) = get_row_and_new_num(&word_lengths, num, symbol);
            println!("length {new_num}:  {hist_row:<5}");
        }
    } else {
        create_vertical_histogram(word_lengths, max_length);
    }
}

// VERTICAL HISTOGRAM
fn create_vertical_histogram(word_lengths: &Vec<usize>, max_length: usize) {
    let (base_row, zipped_rows) = get_zipped_and_base(max_length, word_lengths);
    for row in zipped_rows.iter().rev() {
    let formatted_row = format_row(row);
    println!("{formatted_row}");
    }
    println!("{base_row}");
}

fn format_row(row: &String) -> String {
    let arr: Vec<&str> = row.split("").collect();
    let mut new_arr = Vec::new();
    for (i, item) in arr.into_iter().enumerate() {
        if i != 0 {
            new_arr.push(String::from(item) + "  ");
        } else {
            new_arr.push(String::from(item));
        }
    }
    new_arr.into_iter().collect::<Vec<String>>().join("")
}

fn get_zipped_and_base(max_length: usize, word_lengths: &Vec<usize>) -> (String, Vec<String>) {
    let base_row = make_base_row(max_length);
    let all_rows = get_all_rows(word_lengths, max_length);
    let max_recurrence = get_max_recurrence(&all_rows);
    let zipped_rows = zip_em(all_rows, max_recurrence);
    (base_row, zipped_rows)
}

// COULD PROBABLY USE THE BUILT IN ZIP FUNCTION, BUILDING MY OWN FOR PRACTICE
fn zip_em(words: Vec<String>, max_recurrence: usize) -> Vec<String> {
    let mut zipped_items = Vec::new();
    for i in 0..(max_recurrence + 1) {
        let mut current_item = String::new();
        for word in &words {
            match word.as_str().chars().nth(i) {
                Some(val) => current_item.push_str(String::from(val).as_str()),
                None      => current_item.push(' '),
            }
        }
        zipped_items.push(current_item);
    }
    zipped_items
}

fn get_max_recurrence(words: &Vec<String>) -> usize {
    let mut current_max = 0;
    for word in words {
        let length = word.len();
        current_max = max(length, current_max);
    }
    current_max
}

fn get_all_rows(word_lengths: &Vec<usize>, max_length: usize) -> Vec<String> {
    let mut all_rows = Vec::new();
    for num in 1..(max_length + 1) {
        let symbol = "*";
        let (hist_row, _) = get_row_and_new_num(&word_lengths, num, symbol);
        all_rows.push(hist_row);
    }
    all_rows
}

fn make_base_row(max_length: usize) -> String {
    let mut base_row = String::new();
    for num in 1..(max_length + 1) {
        base_row.push_str(&get_base_row_value(num));
    }
    base_row
}

fn get_base_row_value(num: usize) -> String {
    let new_str = num.to_string() + " ";
    let single_digit_str = String::from("0") + &new_str;
    if num < 10 { single_digit_str } else { new_str }
}

// HORIZONTAL HISTOGRAM
fn get_row_and_new_num(word_lengths: &Vec<usize>, num: usize, symbol: &str) -> (String, String) {
    let count = word_lengths.iter().filter(|x| x == &&num).count();
    let hist_row = get_row(count, symbol);
    let new_num = format_num(num);
    (hist_row, new_num)
}

fn format_num(num: usize) -> String {
    if num < 10 {
        return " ".to_string() + num.to_string().as_str()
    }
    num.to_string()
}

fn get_row(count: usize, symbol: &str) -> String {
    let mut row = String::new();
    for _ in 0..(count) {
        row.push_str(symbol);
    }
    row
}

fn get_max(word_lengths: &Vec<usize>) -> usize {
    let mut current_max = 0;
    for num in word_lengths {
        current_max = max(num.to_owned(), current_max);
    }
    current_max
}

// FUNCS TO GET ARRAY OF FORMATTED WORDS
fn format_words(words: Vec<String>) -> Vec<usize> {
    words.into_iter().map(get_formatted_length).collect()
}


fn get_formatted_length(word: String) -> usize {
    let no_punctuation_word = remove_punctuation(word);
    no_punctuation_word.len()
}

fn remove_punctuation(word: String) -> String {
    let invalid_chars = ['!', '.', ',', ':'];
    word
        .chars()
        .filter(|ch| !invalid_chars.contains(ch))
        .collect()
}

fn get_file_contents(path: &String) -> String {
    let err = "cannot read file.";
    fs::read_to_string(path).expect(err)
}
