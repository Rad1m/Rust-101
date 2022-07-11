// Arrays - Fixed list where elements are the same data types


pub fn run() {
    let mut numbers: [i32; 5] = [1,2,3,4,5];

    // change values
    numbers[2] = 20;

    println!("{:?}", numbers);
    
    // Get single value from array
    println!("First number: {}", numbers[0]);

    // get array length
    println!("Array length: {}", numbers.len());

    // print biggest number
    let mut x = 0;
    for i in numbers {
        if i > x {
            x = i;
        } 
    }
    println!("Biggest number is {}", x);

    // array are stack allocated
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));

    // Get slide
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
}