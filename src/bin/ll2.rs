#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn print_linked_list(head: &Option<Box<ListNode>>) -> String {
    let mut cur = head;
    let mut out = Vec::new();
    while let Some(node) = cur {
        out.push(node.val.to_string());
        cur = &node.next;
    }
    out.push("None".into());
    out.join(" --> ")
}

pub fn reverse_between(
    head: Option<Box<ListNode>>,
    left: i32,
    right: i32,
) -> Option<Box<ListNode>> {
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

// Helpers & test
fn create_linked_list(xs: &[i32]) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0));
    let mut cur = &mut dummy;
    for &v in xs {
        cur.next = Some(Box::new(ListNode::new(v)));
        cur = cur.next.as_mut().unwrap();
    }
    dummy.next
}

fn main() {
    let head = create_linked_list(&[1, 2, 3, 4, 5, 6, 7, 8]);
    println!("Original: {}", print_linked_list(&head));

    let out = reverse_between(head, 2, 4);
    println!("Reversed 2–4: {}", print_linked_list(&out));

    // More tests
    let out = reverse_between(create_linked_list(&[1, 2, 3, 4, 5]), 1, 5);
    println!("Reversed 1–5: {}", print_linked_list(&out));

    let out = reverse_between(create_linked_list(&[5]), 1, 1);
    println!("Single-node: {}", print_linked_list(&out));

    let out = reverse_between(None, 0, 0);
    println!("Empty list: {}", print_linked_list(&out));
}
