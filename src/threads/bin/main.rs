use std::sync::mpsc;
use std::{thread,time};

fn main() {
    let (tx, rx) = mpsc::channel();

    let sender_thread = thread::spawn(move || {
        thread::sleep(time::Duration::from_millis(2000));
        let val = String::from("Ping");
        tx.send(val).unwrap();
    });

    let receiver_thread = thread::spawn(move || {
        let mut run: bool = true;
        while run {
            thread::sleep(time::Duration::from_millis(3000));
            let received = rx.recv().unwrap();
            println!("received: {}", received);
            if received == "Ping"{
                run = false;
            }            
        };
        println!("receiver stopped");
    });

    thread::sleep(time::Duration::from_millis(6000));
}
