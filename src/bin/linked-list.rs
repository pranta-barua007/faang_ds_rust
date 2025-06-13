use std::boxed::Box;

#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

fn create_list(nums: &Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for &val in nums.iter().rev() {
        let mut node = Box::new(ListNode::new(val));
        node.next = head;
        head = Some(node);
    }
    head
}

fn print_list(head: &Option<Box<ListNode>>) {
    let mut current = head;
    while let Some(node) = current {
        print!("{} -> ", node.val);
        current = &node.next;
    }
    println!("None");
}

fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    let mut current = head;
    while let Some(mut node) = current {
        let next = node.next;
        node.next = prev;
        prev = Some(node);
        current = next;
    }
    prev
}

fn reverse_between(head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
    if head.is_none() || left == right {
        return head;
    }

    // Dummy to simplify head-reversal case
    let mut dummy = Box::new(ListNode::new(0));
    dummy.next = head;
    let mut dummy = Some(dummy);

    // 1) Advance prev_of_sub_list to just before `left`
    let mut prev_of_sub_list = &mut dummy;
    for _ in 1..left {
        prev_of_sub_list = &mut prev_of_sub_list.as_mut().unwrap().next;
    }

    // 2) Detach and reverse [left..=right]
    let mut current = prev_of_sub_list.as_mut().unwrap().next.take();
    let mut prev = None;

    // We'll capture the *first* node we reverse as a raw pointer:
    // that's the tail of the reversed sublist.
    let mut tail_ptr: *mut ListNode = std::ptr::null_mut();

    for i in left..=right {
        if let Some(mut node) = current {
            // Before we break the link, if this is the first reversed node,
            // capture its address:
            if i == left {
                tail_ptr = &mut *node;
            }

            // Standard in-place reverse step:
            let next = node.next.take();
            node.next = prev;
            prev = Some(node);
            current = next;
        } else {
            break; // in case `right` exceeds list length
        }
    }

    // 3) Reconnect:
    //    a) before-left → head-of-reversed (prev)
    prev_of_sub_list.as_mut().unwrap().next = prev;

    //    b) tail-of-reversed → current (the remainder)
    if !tail_ptr.is_null() {
        // SAFETY: `tail_ptr` was taken from a Box that we still own,
        // and no other mutable borrows exist now.
        unsafe {
            (*tail_ptr).next = current;
        }
    }

    // 4) Drop dummy and return real head
    dummy.unwrap().next
}

fn main() {
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let list = create_list(&nums);
    print_list(&list);
    let reversed_list = reverse_list(list);
    println!("Reversed:");
    print_list(&reversed_list);
    let list2 = create_list(&nums);
    let rev_btn = reverse_between(list2, 2, 4);
    println!("Reversed 2–4:");
    print_list(&rev_btn)
}
