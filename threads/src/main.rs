use std::{thread, time::Duration};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10{
            println!("Hi number {} from spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // handle.join().unwrap(); // Spawned thread would finish running before the main thread would continue 

    for i in 1..5{
        println!("Hi number {} from main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap(); // Allows the spawned thread to finish running before terminating the main thread

    let v = vec![1, 2, 3];
    // let handle = thread::spawn(|| { // Errors because rust can't tell how long v would be valid
    //     println!("Here's a vector: {:?}", v);
    // });

    let handle = thread::spawn(move || { // Now v has been moved into the closure so now it doesn't exist in the main thread again
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
