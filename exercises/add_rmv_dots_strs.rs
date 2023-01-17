// PROBLEM DESCRIPTION AT BOTTOM OF FILE
// In addtion to solving, I used this an an opportunity to understand lifetimes with structs
use std::io;

struct Msg<'a> {
    welcome: &'a str,
    res: &'a str,
    endln: &'a str,
}

fn main() {
    let msg1 = Msg {
        welcome: "Enter a string without the '.' char (ex: 'test'): ",
        res: "Result separated with dot char: ",
        endln: "\n\n",
    };

    let msg2 = Msg {
        welcome: "Enter a string separated with the '.' char (ex: 't.e.s.t'): ",
        res: "Result with dots removed: ",
        endln: "",
    };

    run_input(msg1, add_dots);
    run_input(msg2, remove_dots);
}

fn run_input(msg: Msg, f: fn(String) -> String) {
    println!("{}", msg.welcome);
    let input = get_input();
    let res = f(input);
    println!("{}{res}{}", msg.res, msg.endln);
}

fn remove_dots(val: String) -> String {
    val.replace(".", "")
}

fn add_dots(val: String) -> String {
    let mut new_val = String::new();
    let trimmed_val = val.trim();
    let last_char = trimmed_val.len() - 1;

    for (i, ch) in trimmed_val.chars().enumerate() {
        if i != last_char {
            new_val += &(ch.to_string() + ".");
        } else {
            new_val += &ch.to_string();
        }
    }
    new_val
}

fn get_input() -> String {
    let mut val = String::new();
    let err = "Could not read input";
    io::stdin().read_line(&mut val).expect(err);
    val
}



// Adding and removing dots
// Write a function named add_dots that takes a string and adds "." in between each letter. 
// For example, calling add_dots("test") should return the string "t.e.s.t".

// Then, below the add_dots function, write another function named remove_dots that 
// removes all dots from a string. For example, calling remove_dots("t.e.s.t") should return "test".

// If both functions are correct, calling remove_dots(add_dots(string)) should 
// return back the original string for any string.

// (You may assume that the input to add_dots does not itself contain any dots.)