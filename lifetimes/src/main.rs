fn main() {
    {
        let string1 = String::from("abcd");
        let string2 = String::from("xyz");
    
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    {
        let string3 = String::from("1234");
        let result;
        {
            let string4 = String::from("890");
            result = longest(string3.as_str(), string4.as_str());
            println!("The longest string is {}", result);
        } // string 4 was dropped here
        // println!("The longest string is {}", result) // Will error because the lifetime of the result would be the same as the lifetime of the string4 and that is all out of scope so it wil error
    }

    {
        let string5 = String::from("mnop");
        let result;
        {
            let string6 = String::from("qrs");
            result = do_first(string5.as_str(), string6.as_str())
        }
        println!("The longest string is {}", result);
        
    }

}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { // Life time is going to be set to whatever has a lower lifetime
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn do_first<'a>(x: &'a str, _y: &str) -> &'a str { // Life time is going to be set to x's life time
    x
}

fn ret_ref<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("Really long string");
    result.as_str() // This reference will fail because at the end of the function result would be out of scope
}

fn ret_ref_correct(x: &str, y: &str) -> String {
    String::from("Really long string") // valid because string type is not a reference
}