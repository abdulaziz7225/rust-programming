use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    
    thread::spawn(move || {
        let values = vec![
            String::from("Good"),
            String::from("morning,"),
            String::from("from"),
            String::from("spawned"),
            String::from("thread"),
        ];

        for val in values {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let values = vec![
            String::from("Warm"),
            String::from("greeting"),
            String::from("to"),
            String::from("all"),
            String::from("Rustaceans"),
        ];

        for val in values {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Received : {received}");
    }
}