// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run () {
    let name = "Radim";
    let mut age = 42;

    age += 1;

    println!("My name is {} and I'm {}", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    let new_age = age;

    // Assign multiple vars
    let (my_name, my_age) = ("Radim", new_age);
    println!("{} is {}", my_name, my_age);
}