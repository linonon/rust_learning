#[test]
fn test_clone() {
    struct Node {
        value: i32,
        next: Option<Box<Node>>,
    }

    impl Node {
        fn new(value: i32) -> Self {
            Node { value, next: None }
        }
    }

    struct List {
        head: Option<Box<Node>>,
    }

    impl List {
        fn new() -> Self {
            List { head: None }
        }

        fn push(&mut self, value: i32) {
            let mut node = Box::new(Node::new(value));
            match self.head.take() {
                Some(head) => {
                    node.next = Some(head);
                    self.head = Some(node);
                }
                None => {
                    self.head = Some(node);
                }
            }
        }

        fn print_all(&mut self) {
            let mut s = String::new();
        }
    }

    let mut n = List::new();

    n.push(1);
    n.push(2);
    n.push(3);

    n.print_all();
}
