mod test;
mod unsafe_impl;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node { value, next: None }))
    }

    pub fn value(&self) -> &T {
        &self.value
    }
}

#[derive(Debug)]
pub struct LinkedList<T> {
    /// head.next 指向头结点
    pub head: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> LinkedList<T>
where
    T: std::fmt::Debug + Clone,
{
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn push(&mut self, value: T) {
        let node = Node::new(value);
        match self.head.clone() {
            Some(head) => {
                // init head.next
                head.borrow_mut().next = Some(node.clone());

                // init head
                self.head = Some(node.clone());

                // init head.next = head
                node.borrow_mut().next = Some(head.clone());
            }
            None => {
                // init head
                self.head = Some(node.clone());

                // init head.next = head
                node.borrow_mut().next = Some(node.clone());
            }
        }
    }

    fn value(&self) -> Option<T> {
        match &self.head {
            Some(head) => Some(head.borrow().value.clone()),
            None => None,
        }
    }

    fn next(&self) -> Self {
        match &self.head {
            Some(head) => LinkedList {
                head: head.borrow().next.clone(),
            },
            None => LinkedList { head: None },
        }
    }
}

fn main() {
    let mut n = LinkedList::new();
    n.push(1);
    n.push(2);
    n.push(3);
    println!("{:?}", n.value().unwrap());
    let n = n.next();
    println!("{:?}", n.value().unwrap());
    let n = n.next();
    println!("{:?}", n.value().unwrap());
    let n = n.next();
    println!("{:?}", n.value().unwrap());
}
