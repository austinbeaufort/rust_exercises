// CHECK IF INPUT IS A PALINDROME
use std::io;

// // FINAL SOLUTION
fn main() {
    println!("Enter a string to check if it's a palindrome: ");
    let input = get_input().trim().to_owned();
    let result = check_palindrome(&input);
    println!("{input}{result}");
}

fn check_palindrome(val: &String) -> String {
    let (msg_is_palindrome, msg_not_palindrome) = get_outputs();
    let rev_chars: String = val.chars().rev().collect();
    if val.eq(&rev_chars) {
        msg_is_palindrome
    } else {
        msg_not_palindrome
    }
}

fn get_input() -> String {
    let err = "Could not read input.";
    let mut val = String::new();
    io::stdin().read_line(&mut val).expect(err);
    val
}

fn get_outputs() -> (String, String) {
    let msg_is_palindrome = String::from(" IS a palindrome!");
    let msg_not_palindrome = String::from(" is NOT a palindrome!");
    (msg_is_palindrome, msg_not_palindrome)
}

// FIRST SOLUTION, THIS WORKS CORRECTLY BUT THERE IS AN EASIER WAY ABOVE.
// fn main() {
//     println!("Enter a string to check if it's a palindrome: ");
//     let input = get_input();
//     let result = check_palindrome(&input);
//     println!("{input}{result}");
// }

// fn check_palindrome(val: &String) -> String {
//     let (msg_is_palindrome, msg_not_palindrome) = get_outputs();
//     let (chars, rev_chars) = clean_vecs(get_chars(&val));
//     println!("{chars:?}");
//     println!("{rev_chars:?}");
//     for (i, _ch) in chars.iter().enumerate() {
//         if chars[i] != rev_chars[i] {
//             return msg_not_palindrome;
//         }
//     }
//     msg_is_palindrome
// }

// fn clean_vecs((chars, rev_chars): (Vec<String>, Vec<String>)) -> (Vec<String>, Vec<String>) {
//     (clean(chars), clean(rev_chars))
// }

// fn clean(vec: Vec<String>) -> Vec<String> {
//     vec.into_iter()
//         .filter(|ch| ch.as_bytes() != "".as_bytes() && ch.as_bytes() != "\n".as_bytes())
//         .collect()
// }

// fn get_chars(val: &String) -> (Vec<String>, Vec<String>) {
//     let chars: Vec<String> = val.split("").map(String::from).collect();
//     let rev_chars = val
//         .split("")
//         .map(String::from)
//         .collect::<Vec<String>>()
//         .into_iter()
//         .rev()
//         .collect();
//     (chars, rev_chars)
// }

// fn get_outputs() -> (String, String) {
//     let msg_is_palindrome = String::from(" IS a palindrome!");
//     let msg_not_palindrome = String::from(" is NOT a palindrome!");
//     (msg_is_palindrome, msg_not_palindrome)
// }

// fn get_input() -> String {
//     let err = "Could not read input.";
//     let mut val = String::new();
//     io::stdin().read_line(&mut val).expect(err);
//     val
// }
