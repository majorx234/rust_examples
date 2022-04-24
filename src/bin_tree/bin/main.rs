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

fn print_tree(root: Option<Box<Node<i32>>>, level: usize) {
    match root {
        Some(node) => {
            print_tree(node.left, level + 1);
            for _ in 0..level {
                print!("  ");
            }
            println!("{:?}", node.value);
            print_tree(node.right, level + 1);
        }
        None => {}
    }
}

fn main() {
    let tree = generate_tree(3);
    // println!("}{:#?}", tree)
    let x = Box::new(69);
    print_tree(tree, 0);
}
