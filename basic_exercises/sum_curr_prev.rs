fn main() {
    let mut current_num = 0;
    let mut previous_num = 0;

    for num in 1..11 {
        let sum = current_num + previous_num;
        println!("Current num: {current_num}, Previous num: {previous_num}, Sum: {sum}");
        current_num = num;
        previous_num = num - 1;
    }
}
