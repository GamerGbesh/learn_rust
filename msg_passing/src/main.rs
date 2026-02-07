use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    thread::spawn(move || {
        let msg = String::from("hi");
        tx.send(msg).unwrap();
        // println!("Msg is {}", msg); // Fails because sending the message takes owner ship of the message

        let vals = vec![
            String::from("Hi"),
            String::from("From"),
            String::from("The"),
            String::from("Thread")
        ];
        
        for val in vals{
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }

    });

    thread::spawn(move || {
        
        let vals = vec![
            String::from("More"),
            String::from("Messages"),
            String::from("For"),
            String::from("You")
        ];
        
        for val in vals{
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }

    });

    for received in rx {
        println!("Got: {}", received);
    } // The loop blocks the main thread so the print statement after only runs after all the threads run

    println!("I'm here");
}
