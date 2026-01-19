fn main() {
    let string_one = String::from("Hello");
    let string_two = String::from("World");
    if let Ok(val) = add(&string_one, &string_two) {
        println!("{}", val);
    }
    println!("{}", string_one);
}   


fn add(num1: &String, num2: &String) -> Result<i32, String> {
    if num1 == "hello" {
        Ok(3)
    }
    else {
        Err(String::from("hi"))
    }
}
