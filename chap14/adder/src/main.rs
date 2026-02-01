use add_one;

fn main() {
    let num = 5;
    let added = add_one::add_one(num);
    println!("{} plus one is {}!", num, added  );
}
