use ds_lib::listnode::ListNode;
use std::ptr;

#[allow(dead_code)]
struct MyListNode {
    val: i32,
    next: *mut MyListNode,
}

#[allow(dead_code)]
impl MyListNode {
    fn new(val: i32) -> MyListNode {
        MyListNode {
            val,
            next: ptr::null_mut(),
        }
    }

    fn new_as_ptr(val: i32) -> *mut MyListNode {
        &mut MyListNode {
            val,
            next: ptr::null_mut(),
        }
    }

    fn as_ptr(&mut self) -> *mut MyListNode {
        self
    }

    fn insert(&mut self, val: i32) {
        let mut cur = self;
        unsafe {
            while !(*cur).next.is_null() {
                cur = &mut *(*cur).next;
            }
            (*cur).next = Box::into_raw(Box::new(MyListNode::new(val)));
        }
    }

    fn get_next(&mut self) -> *mut MyListNode {
        self.next
    }
}

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn get_len(&self, head: *mut MyListNode) -> i32 {
        let mut len = 0;
        let mut cur = head;
        while cur != ptr::null_mut() {
            len += 1;
            unsafe { cur = (*cur).next };
        }
        len
    }

    pub fn show_list(&self, head: *mut MyListNode) {
        let mut cur = head;
        while cur != ptr::null_mut() {
            unsafe {
                print!("{} ", (*cur).val);
                cur = (*cur).next;
            }
        }
        println!();
    }

    pub fn rotate_right(&self, head: *mut MyListNode, k: i32) -> *mut MyListNode {
        if k == 0 {
            return head;
        }

        if head == ptr::null_mut() {
            return head;
        }

        let mut k = k;
        let len = self.get_len(head);
        k %= len;
        let mut p = head.clone();
        let mut q = head.clone();

        unsafe {
            // 先找到第k+1个节点
            // 1 -> 2 -> 3 -> 4 -> 5
            // q              p
            for _ in 0..=k {
                p = (*p).next
            }

            // p和q同时向后移动，直到p到达链表尾部
            // 此时q指向的节点就是倒数第k+1个节点
            //
            // 1 -> 2 -> 3 -> 4 -> 5 -|
            //           q              p
            while p != ptr::null_mut() {
                p = (*p).next;
                q = (*q).next;
            }

            // 1 -> 2 -> 3 -> 4 -> 5 -|
            //           q
            //                p
            p = (*q).next;

            // 1 -> 2 -> 3 -|
            //           q
            (*q).next = ptr::null_mut();

            //                4 -> 5 -|
            //                q
            //                p
            q = p;

            //                4 -> 5 -|
            //                     q
            //                p
            while (*q).next != ptr::null_mut() {
                q = (*q).next;
            }

            //                4 -> 5 -+ 1 -> 2 -> 3 -|
            //                     q
            //                p
            (*q).next = head;
        }

        p
    }

    pub fn rotate_right_v2(&self, h: ListNode<i32>, k: i32) -> ListNode<i32> {
        let mut p = h.clone();
        let mut q = h.clone();

        for _ in 0..=k {
            p.go_next();
        }

        while p.node.is_some() {
            p.go_next();
            q.go_next();
        }

        p = q.next();
        q.set_next(ListNode::none());
        q = p.clone();

        while q.next().is_some() {
            q.go_next();
        }

        q.set_next(h.clone());

        p
    }
}

fn main() {
    case4()
    // case1();
    // case2();
    // case3();
}

#[allow(dead_code)]
fn case1() {
    println!("case1:");
    let mut a = MyListNode::new(1);
    a.insert(2);
    a.insert(3);
    a.insert(4);
    a.insert(5);

    let s = Solution;
    println!("len: {}", s.get_len(a.as_ptr()));
    let b = s.rotate_right(a.as_ptr(), 2);
    s.show_list(b);
}

#[allow(dead_code)]
fn case2() {
    println!("case2:");
    let mut a = MyListNode::new(1);
    a.insert(2);
    a.insert(3);
    a.insert(4);
    a.insert(5);
    a.insert(6);

    let s = Solution;
    println!("len: {}", s.get_len(a.as_ptr()));
    let b = s.rotate_right(a.as_ptr(), 2);
    s.show_list(b);
}

#[allow(dead_code)]
fn case3() {
    println!("case3:");
    let mut a = MyListNode::new(1);
    a.insert(2);
    a.insert(3);
    a.insert(4);
    a.insert(5);
    a.insert(6);
    a.insert(7);

    let s = Solution;
    println!("len: {}", s.get_len(a.as_ptr()));
    let b = s.rotate_right(a.as_ptr(), 6);
    s.show_list(b);
}

#[allow(dead_code)]
fn case4() {
    let n = ListNode::from(vec![1, 2, 3, 4, 5]);
    let s = Solution;
    let n = s.rotate_right_v2(n, 2);
    n.show();

    let n = s.rotate_right_v2(n, 2);
    n.show();
    println!("{}", n.len())
}
