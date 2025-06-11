use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Hi!");
        tx.send(val).unwrap();
        // println!("val = {val}"); // used a val variable in the spawned thread after we’it has been sent down the channel
    });

    let received = rx.recv().unwrap();
    println!("Received message: {received}");
}
