use std::cell::RefCell;
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
    pub fn get_tree_height(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut height = 0;
        let mut current = root.clone();

        while let Some(node) = current {
            if let Some(left_child) = node.borrow().left.clone() {
                height += 1;
                current = Some(left_child);
            } else {
                break; // No left child, exit loop
            }
        }

        height
    }

    pub fn node_exists(
        index_to_find: i32,
        height: i32,
        root: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let mut left = 0;
        let mut right = 2_i32.pow(height as u32) - 1;
        let mut current = root.clone();
        let mut count = 0;

        while count < height {
            let mid = ((left + right) as f64 / 2.0).ceil() as i32;

            if index_to_find >= mid {
                // Go right
                current = current.and_then(|node| node.borrow().right.clone());
                left = mid;
            } else {
                // Go left
                current = current.and_then(|node| node.borrow().left.clone());
                right = mid - 1;
            }
            count += 1;
        }

        current.is_some()
    }

    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let height = Solution::get_tree_height(&root);
        println!("Height: {}", height); // Debug print

        if height == 0 {
            return 1;
        }

        let upper_count = 2_i32.pow(height as u32) - 1;
        println!("Upper count: {}", upper_count); // Debug print

        let mut left = 0;
        let mut right = upper_count;

        while left < right {
            let index_to_find = ((left + right) as f64 / 2.0).ceil() as i32;
            println!(
                "Checking index: {}, left: {}, right: {}",
                index_to_find, left, right
            ); // Debug print

            if Solution::node_exists(index_to_find, height, &root) {
                left = index_to_find;
            } else {
                right = index_to_find - 1;
            }
        }

        upper_count + left + 1
    }
}

fn main() {
    // Build a test tree:
    //         1
    //       /   \
    //      2     3
    //     / \   /  \
    //    4   5 6    7

    let root = Rc::new(RefCell::new(TreeNode::new(1)));
    let left_child = Rc::new(RefCell::new(TreeNode::new(2)));
    let right_child = Rc::new(RefCell::new(TreeNode::new(3)));

    root.borrow_mut().left = Some(left_child.clone());
    root.borrow_mut().right = Some(right_child.clone());

    left_child.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    left_child.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    right_child.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(6))));
    right_child.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(7))));

    let node_count_values = Solution::count_nodes(Some(root));
    println!("Nodes count: {:?}", node_count_values);
}
