use ds_lib::listnode::{ListNode, UnsafeListNode};

pub fn reverse_between(
    mut head: *mut UnsafeListNode,
    left: i32,
    right: i32,
) -> *mut UnsafeListNode {
    if left == 1 && right == 1 {
        return head;
    }

    unsafe {
        if left != 1 {
            (*head).next = reverse_between((*head).next, left - 1, right - 1);
        } else {
            let tail = (*head).next;
            let new_head = reverse_between((*head).next, left, right - 1);

            (*head).next = (*tail).next;
            (*tail).next = head;
            head = new_head;
        }
    }

    head
}

pub fn reverse_between_v2(node: ListNode<i32>, left: i32, right: i32) -> ListNode<i32> {
    let mut node = node;
    if left == 1 && right == 1 {
        return node;
    }

    if left != 1 {
        let next = reverse_between_v2(node.next(), left - 1, right - 1);
        node.set_next(next);
    } else {
        let mut tail = node.next();
        let new_head = reverse_between_v2(node.next(), left, right - 1);

        node.set_next(tail.next());
        tail.set_next(node);

        node = new_head;
    }

    node
}

fn main() {
    let mut head = UnsafeListNode::from(vec![1, 2, 3, 4, 5]);
    head.show();

    let head = reverse_between(head.as_ptr(), 2, 4);
    print!("result: ");
    unsafe { (*head).show() }

    let mut head = ListNode::from(vec![1, 2, 3, 4, 5]);
    head.show();

    let head = reverse_between_v2(head, 2, 4);
    print!("result: ");
    head.show();
}
