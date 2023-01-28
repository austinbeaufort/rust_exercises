fn main() {
    println!("**** MULTIPLICATION TABLE ****");
    for num in 1..13 {
        let mut row = make_row(num);
        row = append_num_to_row(num, row);
        println!("{row}");
    }
}

fn append_num_to_row(num: i32, row: String) -> String {
    let inner_space = if num > 9 { " " } else { "  " };
    num.to_string() + inner_space + &row
}

fn make_row(num: i32) -> String {
    let mut row = String::new();
    for x in 1..11 {
        let val_as_str = get_val_as_str(x, num);
        row.push_str(&val_as_str);
    }
    row
}

fn get_val_as_str(x: i32, num: i32) -> String {
    (x * num).to_string() + " "
}
