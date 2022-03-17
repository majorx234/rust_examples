fn main() {
    for &number in &[1, 1, 2, 3, 5, 8, 13] {
        println!("the next value is {}", number);
    }

    // new in Rust 1.53 by value iterator
    for number in [1, 1, 2, 3, 5, 8, 13] {
        println!("the next value is {}", number);
    }
}
