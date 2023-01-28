use std::io;

fn main() {
    println!("Enter first num: ");
    let num1 = parse(get_input());
    println!("Enter 2nd num: ");
    let num2 = parse(get_input());

    if product_gt_1000(num1, num2) {
        let result = num1 + num2;
        println!("Result of num1 * num2 is greater than 1000, the sum is {result}");
    } else {
        let result = num1 * num2;
        println!("Result of num1 * num2 is less than 1000, the product is {result}");
    };
}

fn product_gt_1000(num1: i32, num2: i32) -> bool {
    num1 * num2 > 1000
}

fn get_input() -> String {
    let error = "Could not read input";
    let mut num = String::new();

    io::stdin().read_line(&mut num).expect(error);
    num
}

fn parse(val: String) -> i32 {
    let error = "Cannot parse value.";
    val.trim().parse().expect(error)
}
