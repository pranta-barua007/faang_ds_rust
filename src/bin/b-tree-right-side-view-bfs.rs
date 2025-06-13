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
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // If root is None, return an empty vector
        if root.is_none() {
            return vec![];
        }

        let mut result = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(root.unwrap());

        while !queue.is_empty() {
            let level_size = queue.len();
            let mut current_node = None;

            // Iterate through all nodes at this level
            for _ in 0..level_size {
                let node = queue.pop_front().unwrap();
                let node_ref = node.borrow();
                current_node = Some(node_ref.val);

                // Push left and right children to queue if they exist
                if let Some(ref left) = node_ref.left {
                    queue.push_back(left.clone());
                }
                if let Some(ref right) = node_ref.right {
                    queue.push_back(right.clone());
                }
            }

            // Push the last node of the level to result
            if let Some(value) = current_node {
                result.push(value);
            }
        }

        result
    }
}

fn main() {
    // Build a test tree:
    //         1
    //       /   \
    //      2     3
    //     / \     \
    //    4   5     6
    //     \
    //      7
    //     /
    //    8

    let root = Rc::new(RefCell::new(TreeNode::new(1)));
    let left_child = Rc::new(RefCell::new(TreeNode::new(2)));
    let right_child = Rc::new(RefCell::new(TreeNode::new(3)));

    root.borrow_mut().left = Some(left_child.clone());
    root.borrow_mut().right = Some(right_child.clone());

    left_child.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    left_child.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    right_child.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(6))));

    let node_7 = Rc::new(RefCell::new(TreeNode::new(7)));
    left_child
        .borrow_mut()
        .left
        .as_ref()
        .unwrap()
        .borrow_mut()
        .right = Some(node_7.clone());

    let node_8 = Rc::new(RefCell::new(TreeNode::new(8)));
    node_7.borrow_mut().left = Some(node_8);

    let right_side_values = Solution::right_side_view(Some(root));
    println!("Right side traversal: {:?}", right_side_values);
}
