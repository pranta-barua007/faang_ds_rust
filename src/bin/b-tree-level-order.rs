use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub struct Solution;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        // If the root is None, then return an empty vector.
        if root.is_none() {
            return vec![];
        }

        // Unwrap the root since we know it exists
        let root = root.unwrap();
        // Initialize results vector.
        let mut result = Vec::new();
        // Create a queue using VecDeque.
        let mut queue = VecDeque::new();
        queue.push_back(root);

        // Continue processing until the queue is empty.
        while !queue.is_empty() {
            // Get the number of nodes at the current level.
            let level_size = queue.len();
            let mut current_level = Vec::with_capacity(level_size);

            // Process all nodes at this level.
            for _ in 0..level_size {
                // Pop the node from the front of the queue.
                let node: Rc<RefCell<TreeNode>> = queue.pop_front().unwrap();
                let node_ref = node.borrow();
                // Add the current node's value to the level vector.
                current_level.push(node_ref.val);

                // If the left child exists, push it to the queue.
                if let Some(ref left) = node_ref.left {
                    queue.push_back(left.clone());
                }
                // If the right child exists, push it to the queue.
                if let Some(ref right) = node_ref.right {
                    queue.push_back(right.clone());
                }
            }
            // Add the current level's values to the results.
            result.push(current_level);
        }

        result
    }
}

fn main() {
    // Build a simple binary tree:
    //         1
    //       /   \
    //      2     3
    //     / \
    //    4   5
    let root = Rc::new(RefCell::new(TreeNode::new(1)));
    let left_child = Rc::new(RefCell::new(TreeNode::new(2)));
    let right_child = Rc::new(RefCell::new(TreeNode::new(3)));
    root.borrow_mut().left = Some(left_child.clone());
    root.borrow_mut().right = Some(right_child.clone());
    left_child.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    left_child.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(5))));

    // Get the level order traversal of the binary tree.
    let levels = Solution::level_order(Some(root));
    println!("Level order traversal: {:?}", levels);
}
