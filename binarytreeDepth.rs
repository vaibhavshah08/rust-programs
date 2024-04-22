//Given a binary tree, implement a function that returns the maximum depth of the tree.

use std::io;

#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }
    
    fn max_depth(root: Option<Box<TreeNode>>) -> i32 {
        match root {
            Some(node) => 1 + Self::max_depth(node.left).max(Self::max_depth(node.right)),
            None => 0,
        }
    }
}

fn main() {
    let mut root = TreeNode::new(3);
    let left = TreeNode::new(9);
    let right = TreeNode::new(20);
    let right_left = TreeNode::new(15);
    let right_right = TreeNode::new(7);

    root.left = Some(Box::new(left));
    root.right = Some(Box::new(right));
    root.right.as_mut().unwrap().left = Some(Box::new(right_left));
    root.right.as_mut().unwrap().right = Some(Box::new(right_right));

    println!("Maximum depth: {}", TreeNode::max_depth(Some(Box::new(root))));
}
