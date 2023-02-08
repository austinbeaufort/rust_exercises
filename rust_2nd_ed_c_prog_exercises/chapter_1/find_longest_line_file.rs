use std::{env, fs};

fn main() {
    let args = env::args().collect();
    check_err(&args);

    let text = read_file(&args[1]);
    let lines: Vec<&str> = text.split('\n').collect();

    let (max_length, longest_line) = get_max_and_line(lines);
    println!("longest line length: {max_length}");
    println!("longest line: {longest_line}");
}

fn get_max_and_line(lines: Vec<&str>) -> (usize, &str) {
    lines.into_iter().fold((0, ""), |acc, line| {
        let max_length = acc.0;
        if line.len() > max_length {
            (line.len(), line)
        } else {
            acc
        }
    })
}

fn read_file(file_name: &String) -> String {
    let err = "could not read input file";
    fs::read_to_string(file_name).expect(err)
}

fn check_err(args: &Vec<String>) {
    if args.len() != 2 {
        panic!("must provide a file name as argument");
    }
}
