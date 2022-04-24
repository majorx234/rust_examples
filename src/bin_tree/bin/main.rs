#[derive(Debug, Default)]
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

fn generate_tree(level: usize) -> Option<Box<Node<i32>>> {
    if level == 0 {
        return None;
    } else {
        Some(Box::new(Node {
            value: level as i32,
            left: generate_tree(level - 1),
            right: generate_tree(level - 1),
        }))
    }
}

fn main() {
    let tree = generate_tree(3);
    println!("{:?}", tree)
}
