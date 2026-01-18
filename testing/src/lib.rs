#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

struct Guess{
    value: i32
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("The value is less than 1");
        }
        if value > 100 {
            panic!("The value is more than 100");
        }
        Guess { value }
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller(){
        let small_rec = Rectangle {height: 2, width: 3};
        let big_rec = Rectangle {height: 5, width: 6};
        assert!(big_rec.can_hold(&small_rec));
    }

    #[test]
    fn smaller_cannot_hold_larger(){
        let small_rec = Rectangle {height: 2, width: 3};
        let big_rec = Rectangle {height: 5, width: 6};
        assert!(!small_rec.can_hold(&big_rec));
    }

    #[test]
    fn test_add_two(){
        assert_eq!(add_two(2), 4);
        assert_eq!(add_two(10), 12);
        assert_ne!(add_two(20), 19);
    }

    #[test]
    fn greeting_contains_name(){
        let result = greeting("James");
        assert!(
            result.contains("James"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    #[should_panic(expected="The value is more than 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    #[should_panic(expected="The value is less than 1")]
    fn less_than_1() {
        Guess::new(-1);
    }

    #[test]
    #[ignore]
    fn expensive_test(){
        let mut counter: u32 = 0;
        for i in 1..=5 {
            counter += i;
        }
        assert!(true);
    }
}
