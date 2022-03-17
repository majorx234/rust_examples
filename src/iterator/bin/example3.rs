fn main() {
    let numbers1 = vec![1, 1, 2, 3, 5, 8, 13];
    println!("the min is {}", numbers1.into_iter().min().unwrap());

    let numbers2 = vec![1, 1, 2, 3, 5, 8, 13];
    println!("the sum is {}", numbers2.into_iter().sum::<i32>());

    // borrowing allows reusing of elements
    let numbers3 = &vec![1, 1, 2, 3, 5, 8, 13];
    println!("the min is {}", numbers3.into_iter().min().unwrap());
    println!("the sum is {}", numbers3.into_iter().sum::<i32>());
}
