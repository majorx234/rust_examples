use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    let sender_thread = thread::spawn(move || {
        thread::sleep_ms(2000);
        let val = String::from("Ping");
        tx.send(val).unwrap();
    });

    let receiver_thread = thread::spawn(move || {
        let mut run: bool = true;
        while run {
            let received = rx.recv().unwrap();
            println!("received: {}", received);
            if received == "Ping"{
                run = false;
            }
        };
        println!("receiver stopped");
    });

    thread::sleep_ms(4000);
}
