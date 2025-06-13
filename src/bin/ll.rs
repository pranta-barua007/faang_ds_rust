// M, N Reversals
// Question: Given a Linked List and numbers m and n, return it back with only positions m to n in reverse.
// Examples:
// ll = 1 → 2 → 3 → 4 → 5, m = 2, n = 4:  1 → 4 → 3 → 2 → 5
// ll = 1 → 4 → 5 → 9 → 11 → 13, m = 1, n = 9:  9 → 5 → 4 → 1 → 11 → 13
// ll = 1 → 2 → 3 → 4 → 5, m = 1, n = 5:  5 → 4 → 3 → 2 → 1
// ll = 5, m = 1, n = 1 → 5
// ll = null, m = 0, n = 0, null

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

// Define the ListNode struct
#[derive(Debug)]
struct ListNode {
    val: i32,
    next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

impl fmt::Display for ListNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

// Function to print the linked list
fn print_linked_list(head: Option<Rc<RefCell<ListNode>>>) -> String {
    let mut current = head;
    let mut nodes = Vec::new();

    while let Some(node) = current {
        nodes.push(node.borrow().val.to_string());
        current = node.borrow().next.clone();
    }

    nodes.push("None".to_string());
    nodes.join(" --> ")
}

// O(n) - Time Complexity
// O(1) - Space Complexity
fn reverse_linked_list(
    start: Rc<RefCell<ListNode>>,
    end: Rc<RefCell<ListNode>>,
) -> (Rc<RefCell<ListNode>>, Rc<RefCell<ListNode>>) {
    let mut prev = None;
    let mut cur_node = Some(start.clone());
    let tail = start.clone();

    while cur_node
        .as_ref()
        .map(|n| !Rc::ptr_eq(n, &end))
        .unwrap_or(false)
    {
        let current = cur_node.unwrap();
        let next_node = current.borrow_mut().next.take();
        current.borrow_mut().next = prev;
        prev = Some(current);
        cur_node = next_node;
    }

    // Process the end node
    let current = cur_node.unwrap();
    current.borrow_mut().next = prev;

    (current, tail)
}

// O(n) - Time Complexity
// O(1) - Space Complexity
fn m_n_reversals(
    head: Option<Rc<RefCell<ListNode>>>,
    left: usize,
    right: usize,
) -> Option<Rc<RefCell<ListNode>>> {
    // Edge cases
    if head.is_none() || left == right {
        return head;
    }

    if let Some(head_node) = head.clone() {
        if head_node.borrow().next.is_none() {
            return head;
        }
    }

    let mut cur_node = head.clone();
    let mut node_before = None;
    let mut node_left = None;

    for i in 1..=right {
        if i == left - 1 {
            node_before = cur_node.clone();
        }
        if i == left {
            node_left = cur_node.clone();
        }

        if let Some(node) = cur_node {
            cur_node = node.borrow().next.clone();
        } else {
            break;
        }
    }

    // If we couldn't find the left or right nodes, return the original list
    if node_left.is_none() || cur_node.is_none() {
        return head;
    }

    let node_after = cur_node.as_ref().unwrap().borrow().next.clone();
    let (start, end) = reverse_linked_list(node_left.unwrap(), cur_node.unwrap());

    end.borrow_mut().next = node_after;

    // Check if left is pointing to head (if so, we do not have any before node!)
    if left == 1 {
        return Some(start);
    }

    node_before.unwrap().borrow_mut().next = Some(start);
    head
}

fn main() {
    // Create a linked list: 8 -> 7 -> 6 -> 5 -> 4 -> 3 -> 2 -> 1
    let node1 = Rc::new(RefCell::new(ListNode::new(1)));
    let node2 = Rc::new(RefCell::new(ListNode::new(2)));
    let node3 = Rc::new(RefCell::new(ListNode::new(3)));
    let node4 = Rc::new(RefCell::new(ListNode::new(4)));
    let node5 = Rc::new(RefCell::new(ListNode::new(5)));
    let node6 = Rc::new(RefCell::new(ListNode::new(6)));
    let node7 = Rc::new(RefCell::new(ListNode::new(7)));
    let node8 = Rc::new(RefCell::new(ListNode::new(8)));

    node1.borrow_mut().next = None;
    node2.borrow_mut().next = Some(node1.clone());
    node3.borrow_mut().next = Some(node2.clone());
    node4.borrow_mut().next = Some(node3.clone());
    node5.borrow_mut().next = Some(node4.clone());
    node6.borrow_mut().next = Some(node5.clone());
    node7.borrow_mut().next = Some(node6.clone());
    node8.borrow_mut().next = Some(node7.clone());

    let ll = Some(node8.clone());

    println!("{}", print_linked_list(ll.clone()));
    println!("{}", print_linked_list(m_n_reversals(ll, 2, 4)));
}
