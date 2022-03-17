pub trait IntoIterator {
    type Item;
    type IntoIter: Iterator;
    fn into_iter(self) -> Self::IntoIter;
}

fn main() {
    let numbers = vec![1, 1, 2, 3, 5, 8, 13];

    //std::vec::Vec implements IntoIterator
    // therefore we can use it in a for loop
    for number in numbers {
        println!("the next value is {}", number);
    }
}
