// Write a function that returns the elements on odd positions in a list.

fn main() {
    let pets: [&str; 6] = ["dog", "cat", "bird", "fish", "rat", "hamster"];
    let odd_pets = get_odd_pets(pets);
    println!("{odd_pets:?}");
}

fn get_odd_pets(pets: [&str; 6]) -> Vec<&str> {
    let mut new_list = Vec::new();
    for i in 0..pets.len() {
        if i % 2 != 0 {
            new_list.push(pets[i]);
        }
    }
    new_list
}
