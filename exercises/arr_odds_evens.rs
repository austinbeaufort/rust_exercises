// Given two lists of numbers, write a program to create a new list such that the 
// new list should contain odd numbers from the first list and even numbers from the second list.

fn main() {
    let list1 = [10, 20, 25, 30, 35];
    let list2 = [40, 45, 60, 75, 90];
    let mut odds = get_odds(list1);
    let mut evens = get_evens(list2);
    odds.append(&mut evens);
    println!("Result List: {odds:?}");
}

fn get_odds(arr: [i32; 5]) -> Vec<i32> {
    arr.into_iter().filter(odd).collect()
}

fn get_evens(arr: [i32; 5]) -> Vec<i32> {
    arr.into_iter().filter(even).collect()
}

fn even(num: &i32) -> bool {
    num % 2 == 0
}

fn odd(num: &i32) -> bool {
    num % 2 != 0
}
