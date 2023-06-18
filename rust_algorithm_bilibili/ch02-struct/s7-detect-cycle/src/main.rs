use std::cell::RefCell;

use ds_lib::listnode::{ListNode, UnsafeListNode};

struct Solution;

impl Solution {
    pub fn detect_cycle(head: &mut UnsafeListNode) -> Option<&UnsafeListNode> {
        let mut p = head.as_ptr();
        let mut q = head.as_ptr();

        while !q.is_null() && unsafe { !(*q).next.is_null() } {
            unsafe {
                p = (*p).next;
                q = (*(*q).next).next;
            }
            if p == q {
                break;
            }
        }

        if q.is_null() || unsafe { (*q).next.is_null() } {
            return None;
        }

        p = head.as_ptr();
        unsafe {
            while p != q {
                p = (*p).next;
                q = (*q).next;
            }
        }

        return Some(unsafe { &*p });
    }

    pub fn detect_cycle_v2(head: ListNode<i32>) -> Option<ListNode<i32>> {
        let mut p = head.clone();
        let mut q = head.clone();

        while q.is_some() && q.next().is_some() {
            p.go_next();
            q.go_next();
            q.go_next();

            if p == q {
                break;
            }
        }

        if q.is_none() || q.next().is_none() {
            return None;
        }

        p = head.clone();

        while p != q {
            p.go_next();
            q.go_next();
        }

        Some(p)
    }
}

fn main() {
    let mut n = UnsafeListNode::from(vec![3, 2, 0, 4]);
    let n2 = n.get_first_with_val(2).unwrap().as_ptr();
    let n4 = n.get_first_with_val(4).unwrap().as_ptr();
    unsafe {
        (*n4).set_next(n2);
    }

    n.show();

    let cycle_entry = Solution::detect_cycle(&mut n).unwrap();
    assert_eq!(std::ptr::eq(n2, cycle_entry), true);
    cycle_entry.show();

    let mut n = ListNode::from(vec![3, 2, 0, 4]);
    let n2 = n.find_first_node_with_value(2).unwrap();
    let mut n4 = n.find_first_node_with_value(4).unwrap();
    n4.set_next(n2);

    let cycle_entry = Solution::detect_cycle_v2(n).unwrap();
    cycle_entry.show();
}
