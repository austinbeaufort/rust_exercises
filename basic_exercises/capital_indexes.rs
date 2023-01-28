// PROBLEM DESCRIPTION AT BOTTOM OF FILE
use std::io;

fn main() {
    println!("Enter word to return capital index list: ");
    let val = get_input();
    let index_list = get_index_list(val);
    println!("Index list of capital chars: {index_list:?}");
}

fn get_index_list(val: String) -> Vec<usize> {
    let mut index_list = Vec::new();
    for (i, ch) in val.chars().enumerate() {
        if ch.is_uppercase() {
            index_list.push(i);
        }
    }
    index_list
}

fn get_input() -> String {
    let err = "Cannot read input.";
    let mut val = String::new();
    io::stdin().read_line(&mut val).expect(err);
    val
}

// Write a function named capital_indexes.
// The function takes a single parameter, which is a string.
// Your function should return a list of all the indexes in the string
// that have capital letters.

// For example, calling capital_indexes("HeLlO") should
// return the list [0, 2, 4].
