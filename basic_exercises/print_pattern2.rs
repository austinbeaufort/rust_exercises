
fn main() {
    for num in (0..6).rev() {
        let row = get_row(num);
        println!("{row}");
    }
}

fn get_row(num: i32) -> String {
    let mut row = String::new();
    for _ in 0..num {
        row.push_str("* ");
    }
    row
}




// PRINT THE FOLLOWING PATTERN

// * * * * *  
// * * * *  
// * * *  
// * *  
// *