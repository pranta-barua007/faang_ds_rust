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
    pub fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
        // Pre-Order Traversal
        if let Some(n) = node {
            let val = n.borrow().val as i64;

            if val <= min || val >= max {
                return false;
            }

            if n.borrow().left.is_some() {
                if !Solution::dfs(&n.borrow().left, min, val) {
                    return false;
                }
            }

            if n.borrow().right.is_some() {
                if !Solution::dfs(&n.borrow().right, val, max) {
                    return false;
                }
            }
        }
        true
    }

    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }
        return Solution::dfs(&root, i64::MIN, i64::MAX);
    }
}

fn main() {
    // Build a binary search tree:
    //         10
    //       /   \
    //      5     18
    //     / \   /  \
    //    3   8 12   25

    let root = Rc::new(RefCell::new(TreeNode::new(10)));
    let left_child = Rc::new(RefCell::new(TreeNode::new(5)));
    let right_child = Rc::new(RefCell::new(TreeNode::new(18)));

    root.borrow_mut().left = Some(left_child.clone());
    root.borrow_mut().right = Some(right_child.clone());

    left_child.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    left_child.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(8))));
    right_child.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(12))));
    right_child.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(25))));

    let is_valid = Solution::is_valid_bst(Some(root));
    println!("BST valid status: {:?}", is_valid);
}
