fn main() {
    let mut x = 5;
    println!("x is: {}", x);
    
    {
        let x = 2;
        println!("x is: {}", x);
    }

    x += 3;
    println!("x is: {}", x);
    println!("another line");

    let name = "Frank";

    say_hello(name);
    
}


fn say_hello(name: &str){
    println!("hello {}", name);
}