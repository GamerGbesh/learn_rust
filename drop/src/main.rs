struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer{
        data: String::from("My stuff")
    };

    let d = CustomSmartPointer {
        data: String::from("Other stuff")
    };

    println!("CustomSmartPointers created.");
    // c.drop(); Errors because Rust doesn't like this since it would call drop again at the end
    drop(c); // Use this instead
    println!("C dropped early");

}