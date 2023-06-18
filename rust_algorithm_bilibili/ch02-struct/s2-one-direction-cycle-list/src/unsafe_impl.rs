use std::ptr;

#[derive(Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: *mut ListNode,
}

#[allow(dead_code)]
impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode {
            next: ptr::null_mut(),
            val,
        }
    }

    pub fn set_next(&mut self, next: *mut ListNode) {
        self.next = next;
    }

    pub fn get_next(&self) -> &mut ListNode {
        unsafe { &mut *self.next }
    }

    pub fn as_ptr(&self) -> *mut ListNode {
        self as *const ListNode as *mut ListNode
    }

    pub fn value(&self) -> &i32 {
        &self.val
    }
}

#[allow(dead_code)]
pub fn show_list(head: &mut ListNode) {
    let mut n = head;
    let max = 10;
    let mut i = 0;
    // head.next = head
    while n.as_ptr() != ptr::null_mut() {
        print!("{:?},", n.value());
        n = n.get_next();
        i += 1;
        if i > max {
            break;
        }
    }
}

#[test]
fn test_unsafe_list() {
    let node1 = &mut ListNode::new(1);
    let node2 = &mut ListNode::new(2);
    let node3 = &mut ListNode::new(3);
    let node4 = &mut ListNode::new(4);

    node1.set_next(node2);
    node2.set_next(node3);
    node3.set_next(node4);

    show_list(node1);
    println!();

    node4.set_next(node1);
    show_list(node4)
}
