use std::io;

fn main() {
    println!("========== INPUT PARSER ==========");
    let inputs = get_inputs();
    let results = remove_blank_lines(trim_inputs(inputs));
    println!("========== RESULTS WITH REMOVED EMPTY INPUTS AND TRIMMED END LINES ==========");
    results.iter().for_each(|line| println!("{}", line));
}

fn remove_blank_lines(lines: Vec<String>) -> Vec<String> {
    lines.into_iter().filter(|line| !line.is_empty()).collect::<Vec<String>>()
}
fn trim_inputs(inputs: Vec<String>) -> Vec<String> {
    inputs.into_iter().map(trim_str_end).collect::<Vec<String>>()
}

fn trim_str_end(input: String) -> String {
    String::from(input.trim_end())
}

fn get_inputs() -> Vec<String> {
    let mut inputs = Vec::new();
    for i in 1..6 {
        println!("Enter input {i}: ");
        inputs.push(get_input());
    }
    inputs
}

fn get_input() -> String {
    let mut val = String::new();
    let err = "could not read input";
    io::stdin().read_line(&mut val).expect(err);
    val
}
