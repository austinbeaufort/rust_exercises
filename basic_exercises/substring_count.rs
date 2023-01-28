// HOW MANY TIMES DOES "FRANK" SUBSTRING APPEAR IN THE GIVEN SENTENCE?
fn main() {
    let str_x = "Frank is good developer. Frank is a writer.";
    let count = split_and_count(str_x);
    println!("Frank appears {count} times.");
}

fn split_and_count(val: &str) -> i32 {
    let items: Vec<&str> = val.split(" ").collect();
    count(items)
}

fn count(items: Vec<&str>) -> i32 {
    let mut count = 0;
    for item in items {
        if item == "Frank" {
            count += 1;
        }
    }
    count
}