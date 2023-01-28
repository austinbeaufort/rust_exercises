// print list in reverse order using a loop

fn main() {
    let nums = [10, 20, 30, 40, 50, 60, 70, 80];
    let rev_arr: Vec<&i32> = nums.iter().rev().collect();
    for num in rev_arr {
        println!("{num}");
    }
}