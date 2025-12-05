fn main() {
    let some_value = Some(3); // Type was inferred as a result of optional_function's val param
    let optional_val = optional_function(some_value);
    match optional_val {
        Some(3) => println!("{}", 3),
        Some(x) => println!("It's not a 3 it's a {}", x),
        None => println!("No value was returned")
    }

    if let Some(3) = some_value {
        println!("Three")
    }
}

fn optional_function(val: Option<i32>) -> Option<i32> {
    match val {
        Some(val) => Some(val),
        None => None
    }
}
