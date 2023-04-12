use std::cmp;

#[derive(Debug)]
struct Node {
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn max_depth(node: &Option<Box<Node>>) -> i32 {
    match node {
        None => 0,
        Some(node) => {
            let left_depth = max_depth(&node.left);
            let right_depth = max_depth(&node.right);
            cmp::max(left_depth, right_depth) + 1
        }
    }
}

fn main() {
    // Build the following binary tree:
    //      1
    //     / \
    //    2   3
    //   / \   \
    //  4   5   6
    let tree = Node {
        val: 1,
        left: Some(Box::new(Node {
            val: 2,
            left: Some(Box::new(Node {
                val: 4,
                left: None,
                right: None,
            })),
            right: Some(Box::new(Node {
                val: 5,
                left: None,
                right: None,
            })),
        })),
        right: Some(Box::new(Node {
            val: 3,
            left: None,
            right: Some(Box::new(Node {
                val: 6,
                left: None,
                right: None,
            })),
        })),
    };
    
    println!("Max depth of tree: {}", max_depth(&Some(Box::new(tree))));
}

