#[derive(Debug, Default)]
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

fn generate_tree(level: usize, counter: &mut i32) -> Option<Box<Node<i32>>> {
    if level == 0 {
        return None;
    } else {
        *counter += 1;
        Some(Box::new(Node {
            value: *counter as i32,
            left: generate_tree(level - 1, counter),
            right: generate_tree(level - 1, counter),
        }))
    }
}

fn invert_tree(root: &Option<Box<Node<i32>>>) -> Option<Box<Node<i32>>> {
    match root {
        Some(node) => Some(Box::new(Node {
            value: node.value,
            left: invert_tree(&node.right),
            right: invert_tree(&node.left),
        })),
        None => None,
    }
}

fn print_tree(root: &Option<Box<Node<i32>>>, level: usize) {
    match root {
        Some(node) => {
            print_tree(&node.left, level + 1);
            for _ in 0..level {
                print!("  ");
            }
            println!("{:?}", node.value);
            print_tree(&node.right, level + 1);
        }
        None => {}
    }
}

fn main() {
    let mut counter: i32 = 0;
    let tree = generate_tree(3, &mut counter);
    // println!("}{:#?}", tree)
    print_tree(&tree, 0);
    let itree = invert_tree(&tree);
    print_tree(&itree, 0);
}
