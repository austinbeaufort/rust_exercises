fn main() {
    for i in 1..6 {
        let row = get_row(i);
        println!("{row}");
    }
}

fn get_row(num: i32) -> String {
    let mut row = String::new();
    let num_str = num.to_string();

    for _i_ in 0..num {
        row.push_str(&num_str);
        row.push_str(" ");
    }
    row
}

// PRINT THIS PATTERN
// 1
// 2 2
// 3 3 3
// 4 4 4 4
// 5 5 5 5 5
