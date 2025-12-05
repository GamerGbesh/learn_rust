use std::io;

fn main() {
    println!("Enter something to be printed: ");
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed");

    println!("{input}");
}
