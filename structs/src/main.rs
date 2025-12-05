struct User{
    username: String,
    email: String,
    age: u8,
    active: bool
}

fn main() {
    println!("Creating my first struct!");
    let phil = build_user(String::from("Philemon"), String::from("gbesh.12@gmail.com"), 18, true);
    println!("user details: name: {}, email: {}, age: {}, active: {}", phil.username, phil.email, phil.age, phil.active);
    

}

fn build_user(username: String, email: String, age: u8, active: bool) -> User{
    User{
        username,
        email,
        age,
        active
    }
}
