use stack_lib::Stack;

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

// WIP
fn invert_tree_iterative(root: &Option<Box<Node<i32>>>) -> Option<Box<Node<i32>>> {
    let max_size: usize = 32;
    let mut tree_stack: Stack<&Option<Box<Node<i32>>>> = Stack::new(max_size);
    let mut new_tree_stack: Stack<&Option<Box<Node<i32>>>> = Stack::new(max_size);
    tree_stack.push(&root);
    'stack: loop {
        let element = tree_stack.pop();
        match element {
            Some(stack_node) => match stack_node {
                Some(node) => {
                    tree_stack.push(&node.right);
                    tree_stack.push(&node.left);
                    new_tree_stack.push(Some(Box::new(Node {
                        value: node.value,
                        left: None,
                        right: None,
                    })));
                }
                None => {}
            },
            None => {}
        };
        if tree_stack.is_empty() {
            break 'stack;
        }
    }
    let last = new_tree_stack.pop();
    match last {
        Some(last_node) => *last_node,
        None => None,
    }
}

fn print_tree(root: &Option<Box<Node<i32>>>, level: usize) {
    match root {
        Some(node) => {
            print_tree(&node.right, level + 1);
            for _ in 0..level {
                print!("  ");
            }
            println!("{:?}", node.value);
            print_tree(&node.left, level + 1);
        }
        None => {}
    }
}

fn print_tree_iterative(root: &Option<Box<Node<i32>>>, level: usize) {
    let max_size: usize = 32;
    let mut node_stack: Stack<(i32, u32)> = Stack::new(max_size);
    let mut tree_stack: Stack<(&Option<Box<Node<i32>>>, u32)> = Stack::new(max_size);
    tree_stack.push((&root, 0));
    'stack: loop {
        let element = tree_stack.pop();
        match element {
            Some(stack_elem) => {
                let (stack_node, deep) = stack_elem;
                match stack_node {
                    Some(node) => {
                        tree_stack.push((&node.right, deep + 1));
                        tree_stack.push((&node.left, deep + 1));
                        node_stack.push((node.value, deep));
                    }
                    None => {
                        let value_opt = node_stack.pop();
                        match value_opt {
                            Some(value_elem) => {
                                let (value, deepness) = value_elem;
                                for _ in 0..deepness {
                                    print!("  ");
                                }
                                println!("{:?}", value);
                            }
                            None => {}
                        };
                    }
                }
            }
            None => {}
        };
        if tree_stack.is_empty() {
            break 'stack;
        }
    }
}

fn main() {
    let mut counter: i32 = 0;
    let tree = generate_tree(3, &mut counter);
    // println!("}{:#?}", tree)
    print_tree(&tree, 0);
    let itree = invert_tree(&tree);
    print_tree(&itree, 0);
    println!("####################");
    print_tree_iterative(&tree, 0);
}
