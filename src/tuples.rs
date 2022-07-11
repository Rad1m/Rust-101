// Tuples group together values of different types
// Max 12 elements

pub fn run () {
    let person: (&str, &str, &str) = ("Radim", "Zurich", "Switzerland");

    println!("{} is from {} {}", person.0, person.1, person.2);
}