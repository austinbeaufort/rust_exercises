// impl_and_struct

struct Calc {}

impl Calc {
    fn add(num1: i32, num2: i32) -> i32 {
        num1 + num2
    }

    fn subtact(num1: i32, num2: i32) -> i32 {
        num1 - num2
    }

    fn multiply(num1: i32, num2: i32) -> i32 {
        num1 * num2
    }

    fn divide(num1: i32, num2: i32) -> i32 {
        num1 / num2
    }
}

fn main() {
    println!("4 + 5 = {}", Calc::add(4, 5));
    println!("4 - 5 = {}", Calc::subtact(4, 5));
    println!("4 * 5 = {}", Calc::multiply(4, 5));
    println!("200 / 10 = {}", Calc::divide(200, 10));
}
