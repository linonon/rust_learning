use ds_lib::listnode::*;

struct Solution;

impl Solution {
    pub fn remove_nth_from_end(head: *mut UnsafeListNode, k: i32) -> *mut UnsafeListNode {
        let mut dummy_head = UnsafeListNode::new(0);

        let mut p = dummy_head.as_ptr().clone();
        let mut q = p.clone();

        dummy_head.next = head;

        unsafe {
            for _ in 0..=k {
                q = (*q).next;
            }

            while !q.is_null() {
                p = (*p).next;
                q = (*q).next;
            }

            (*p).next = (*(*p).next).next;
        }

        dummy_head.next
    }

    pub fn remove_nth_from_end_v2(head: ListNode<i32>, k: i32) -> ListNode<i32> {
        let mut dummy_head = ListNode::new(0);
        dummy_head.set_next(head.clone());

        let mut p = dummy_head.clone();
        let mut q = p.clone();

        for _ in 0..=k {
            p.go_next();
        }

        while p.node.is_some() {
            p.go_next();
            q.go_next();
        }

        p = q.clone();
        p.go_next();
        p.go_next();

        q.set_next(p.clone());

        dummy_head.next()
    }
}

fn main() {
    let mut n = UnsafeListNode::from(vec![1, 2, 3, 4, 5]);

    let n = Solution::remove_nth_from_end(n.as_ptr(), 3);
    unsafe {
        (*n).show();
    }

    let n = ListNode::from(vec![1, 2, 3, 4, 5]);
    let n = Solution::remove_nth_from_end_v2(n, 3);
    n.show();
}
