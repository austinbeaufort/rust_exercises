// IF MULTIPLE BLANKS, REPLACE WITH JUST ONE BLANK
fn main() {
    let sentence = String::from("This is        a sentence     with extra  spaces    .");
    let formatted_sentence = remove_last_space(remove_extra_spaces(sentence));
    println!("{formatted_sentence}");
}

fn remove_extra_spaces(val: String) -> String {
    val
        .split_whitespace()
        .map(str::to_string)
        .collect::<Vec<String>>()
        .join(" ")
}

fn remove_last_space(mut val: String) -> String {
    let last_space_index = val.len() - 2;
    val.remove(last_space_index);
    val
}
