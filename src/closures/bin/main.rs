fn thread_function_with_parameters(value: u32) -> () {
    println!("thread_function_with_parameters: {}", value);
}

fn main() {
    let test_value1: u32 = 42;

    // thread with closure
    let closure_test_thread1: std::thread::JoinHandle<()> = std::thread::spawn(move || {
        println!("test_value1: {}", test_value1);
    });
    closure_test_thread1.join().unwrap();

    // thread with closure and thread function with parameters
    let test_value2: u32 = 43;
    let closure_test_thread2: std::thread::JoinHandle<()> = std::thread::spawn(move || {
        thread_function_with_parameters(test_value2);
    });
    closure_test_thread2.join().unwrap();
}
