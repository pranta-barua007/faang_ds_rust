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
    pub fn dfs(node: Option<Rc<RefCell<TreeNode>>>, level: usize, result: &mut Vec<i32>) {
        if let Some(node) = node {
            if level >= result.len() {
                result.push(node.borrow().val);
            }
            Solution::dfs(node.borrow().right.clone(), level + 1, result);
            Solution::dfs(node.borrow().left.clone(), level + 1, result);
        }
    }
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // If root is None, return an empty vector
        if root.is_none() {
            return vec![];
        }

        let mut result = Vec::new();
        // Start DFS from the root node at level 0
        Solution::dfs(root, 0, &mut result);
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
