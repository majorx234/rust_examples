fn thread_function_with_parameters(value: u32) -> () {
    println!("thread_function_with_parameters: {}", value);
}

fn thread_function_with_function_pointer_parameter(
    value: u32,
    f_closure: impl Fn(u32) -> u32,
) -> () {
    let new_value: u32 = f_closure(value);
    println!(
        "thread_function_with_function_pointer_parameter: {}",
        new_value
    );
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

    //thread with closure and thread function with function trait as parameter
    let test_value3: u32 = 44;
    let test_value4: u32 = 45;
    let test_closure = move |a: u32| a + test_value4;

    let closure_test_thread3: std::thread::JoinHandle<()> = std::thread::spawn(move || {
        thread_function_with_function_pointer_parameter(test_value3, test_closure);
    });
    closure_test_thread3.join().unwrap();
}
