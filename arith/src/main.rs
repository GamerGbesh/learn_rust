use std::io;

fn main() {
    let x: f32 = 20.0;
    let y: f32 = 60.0;

    println!("{}", x / y);

    // Types must be the same before going through maths
    // Make sure the result isn't bigger than the range of the data type or it will cause an overflow
    // Results would also be the same type

    let x: i64 = 200;
    let y: i32 = 10;

    println!("{}", x / (y as i64)); // Type casting i32 to i64

    let a: i32 = i32::MAX;
    let b: i8 = a as i8; // Overflow occurs here not caught by the compiler as an error but it happens
    println!("{}", b);
    println!("a: {}", a);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");

    println!("{}", input); 

    let int_input: i64 = input.trim().parse().unwrap();

    println!("{}", int_input);
}
