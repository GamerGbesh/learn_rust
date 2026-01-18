struct Important<'a> {
    part: &'a str
}

impl<'a> Important<'a> {
    fn return_part(&self, announcement: &str) -> &str{
        self.part   
    }
}

fn main() {
    let novel = String::from("Call me. And it never happend!");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let important = Important {part: first_sentence}; // If first_sentence goes out of scope, important will also go out of scope
}
