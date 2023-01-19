// Fix the error below with least amount of modification
// fn main() {
//     let (mut x, y) = (1, 2);
//     x += 2;

//     assert_eq!(x, 3);
//     assert_eq!(y, 2);

//     println!("Success!");
// }




// fix unused variable warning
// fn main() {
//     let _x = 1; 
// }

// Remove a line in the code to make it compile
// fn main() {
//     let mut _x: i32 = 1;
//     _x = 7;
//     // Shadowing and re-binding
//     let _x = _x; 
//     let _y = 4;
//     // Shadowing
//     let _y = "I can also be bound to text!"; 

//     println!("Success!");
// }



// Only modify `assert_eq!` to make the `println!` work(print `42` in terminal)
// fn main() {
//     let x: i32 = 5;
//     {
//         let x = 12;
//         assert_eq!(x, 12);
//     }

//     assert_eq!(x, 5);
//     let x = 42;
//     println!("{}", x); // Prints "42".
// }



// Fix the error with the use of define_x
// fn main() {
//     let x = define_x();
//     println!("{}, world", x); 
// }

// fn define_x() -> String {
//     let x = String::from("hello");
//     x
// }



// Fix the error below with least amount of modification
// fn main() {
//     let x: i32 = 10;
//     let y: i32 = 5;
//     println!("The value of x is {} and value of y is {}", x, y);
//     println!("The value of x is {} and value of y is {}", x, y); 
// }


// fn main() {
//     let mut x = 1;
//     x += 2; 
    
//     assert_eq!(x, 3);
//     println!("Success!");
// }