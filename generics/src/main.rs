struct Point<T> {
    x: T,
    y: T
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f64> {
    fn y(&self) -> f64 {
        self.y
    }
}


struct DifPoint<T, U> {
    x: T,
    y: U
}

impl <T, U> DifPoint<T, U> {
    fn mixup<V, W>(self, other: DifPoint<V, W>) -> DifPoint<T, W> {
        DifPoint {
            x: self.x,
            y: other.y
        }
    }
}

fn main() {
    let num_list = vec![1, 20, 30, 100, 50];
    let largest = get_largest(num_list);
    println!("The largest element was {}", largest);

    let char_list = vec!['a', 'b', 'c', 'Z', 'z'];
    let large_char = generic_get_largest(char_list);
    println!("The largest element was {}", large_char);


    let p1 = Point {x: 5, y: 20};
    println!("The reference is {}", p1.x());
    // p1.y() // This doesn't exist for p1 because it doesn't fit the type f64

    let p2 = Point {x:10.2, y: 2.3};
    println!("The references are {} and {}", p2.x(), p2.y()); // This works because the type is f64 and a trait is implemented for that


    let p3 = DifPoint {x: 5, y: 10.4};
    let p4 = DifPoint {x: "hello", y: 'c'};
    let p5 = p3.mixup(p4);

    println!("So here is p5 {} and {}", p5.x, p5.y);
}


fn get_largest(list: Vec<i32>) -> i32 {
    let mut largest = list[0];

    for num in list {
        if num > largest {
            largest = num;
        }
    }

    largest
}

fn generic_get_largest<T: PartialOrd + Copy> (list: Vec<T>) -> T {
    let mut largest = list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
