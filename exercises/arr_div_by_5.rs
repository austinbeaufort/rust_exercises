fn main() {
    let arr = [10, 3, 7, 15, 35, 106, 105];
    println!("Divisible by 5");
    print_div_by_5(arr);
}

fn print_div_by_5(arr: [i32; 7]) {
    for num in arr.iter() {
        if num % 5 == 0 {
            println!("{num}");
        }
    }
}
