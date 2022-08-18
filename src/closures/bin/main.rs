fn main() {
    let test1: u32 = 42;
    let closure_test_thread1: std::thread::JoinHandle<()> = std::thread::spawn(move || {
        println!("test1: {}", test1);
    });
    closure_test_thread1.join().unwrap();
}
