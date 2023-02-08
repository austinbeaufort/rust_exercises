use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    check_args(&args);

    let path = &args[1];
    let file_text = read_file(path);
    let lines_over_80 = get_lines_over_80(&file_text);
    println!("============LINES OVER 80 CHARS ============");
    lines_over_80.iter().for_each(|line| println!("{line}"));
}

fn get_lines_over_80(file_text: &str) -> Vec<&str> {
    file_text
        .split('\n')
        .filter(|line| line.len() > 80)
        .collect::<Vec<&str>>()
}

fn read_file(path: &String) -> String {
    let err = "cannot read file";
    fs::read_to_string(path).expect(err)
}

fn check_args(args: &Vec<String>) {
    if args.len() != 2 {
        let err = "must include a file path as argument when running the program";
        panic!("{}", err);
    }
}
