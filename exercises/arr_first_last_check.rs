fn main() {
    let arr = [6, 2, 3, 4, 5, 6];
    let arr2 = [1, 2, 3, 4, 5, 6];
    let res1 = same_first_last(arr);
    let res2 = same_first_last(arr2);
    println!("{res1} :::: {res2}");
}

fn same_first_last(arr: [i32; 6]) -> String {
    let end_index = arr.len() - 1;
    if arr[0] == arr[end_index] {
        s("SAME")
    } else {
        s("NOPE")
    }
}

fn s(val: &str) -> String {
    String::from(val)
}
