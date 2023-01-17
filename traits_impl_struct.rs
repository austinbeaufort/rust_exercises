// PROBLEM DETAILS AT BOTTOM OF FILE
use std::io;

#[derive(Copy, Clone)]
struct InputOutputString<'a> {
    string: &'a str,
}

impl InputOutputString {
    fn set_string(&mut self) {
        let mut val = String::new();
        let err = "Cannot get input";
        io::stdin().read_line(&mut val).expect(err);
        self.string = &val;
    }

    fn get_string(&self) -> &str {
        self.string
    }

    fn print_string(&self) {
        println!("{}", self.string.to_uppercase());
    }
}

fn main() {
    let mut io_string = InputOutputString { string: "" };
    io_string.set_string();
    io_string.get_string();
}

// Define a struct which has at least two methods:
// get_string: to get a string from console input
// print_string: to print the string in upper case.
// Also please include simple test function to test the class methods.