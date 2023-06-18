fn main() {
    println!("Hello, world!");
}

#[allow(unused)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[derive(Clone)]
struct Node {
    data: usize,
    next: Option<Box<Node>>,
}

#[allow(unused)]
impl Node {
    fn new(data: usize) -> Node {
        Node { data, next: None }
    }

    fn insert(&mut self, data: usize, pos: usize) {
        if pos == 0 {
            let new_node = Node {
                data,
                next: Some(Box::new(self.clone())),
            };
            *self = new_node;
        } else {
            let mut current_node = self;
            for _ in 0..pos - 1 {
                if let Some(ref mut next_node) = current_node.next {
                    current_node = next_node;
                } else {
                    panic!("Out of range");
                }
            }
            let new_node = Node {
                data,
                next: current_node.next.take(),
            };
            current_node.next = Some(Box::new(new_node));
        }
    }

    /// using dummy head
    fn insert_v2(&mut self, pos: usize, data: usize) {
        let mut dummy_head = Node {
            data: 0,
            next: Some(Box::new(self.clone())),
        };
        let mut current = &mut dummy_head;
        for _ in 0..pos {
            current = match current.next {
                Some(ref mut node) => node,
                None => break,
            };
        }
        let new_node = Node {
            data,
            next: current.next.take(),
        };
        current.next = Some(Box::new(new_node));
        *self = *dummy_head.next.unwrap();
    }

    fn prepend(&mut self, data: usize) {
        let current_node = std::mem::replace(self, Node::new(0));
        let new_node = Node {
            data,
            next: Some(Box::new(current_node)),
        };
        *self = new_node;
    }

    fn push(&mut self, data: usize) {
        let mut current_node = self;
        while let Some(ref mut next_node) = current_node.next {
            current_node = next_node;
        }
        current_node.next = Some(Box::new(Node::new(data)))
    }

    fn stringify(&self) -> String {
        let mut string = String::new();
        let mut current_node = self;
        while let Some(ref next_node) = current_node.next {
            string.push_str(&format!("{} -> ", current_node.data));
            current_node = next_node;
        }
        string.push_str(&format!("{}", current_node.data));
        string
    }

    fn find(&self, data: usize) -> bool {
        let (found, pos) = self.find_inner(data);
        if found {
            self.print_find_result(pos);
        };
        found
    }

    fn find_inner(&self, data: usize) -> (bool, usize) {
        let mut current_node = self;
        let mut pos = 0;
        while let Some(ref next_node) = current_node.next {
            if current_node.data == data {
                return (true, pos);
            }
            current_node = next_node;
            pos += 1;
        }

        if current_node.data == data {
            (true, pos)
        } else {
            (false, 0)
        }
    }

    fn print_find_result(&self, pos: usize) {
        println!("{}", self.stringify());
        let len = pos * 5;
        let mut spaces = String::new();
        for _ in 0..len {
            spaces.push(' ');
        }
        spaces.push_str("↑");

        println!("{}", spaces);
    }
}

#[test]
fn test() {
    let mut node = Node::new(1);
    node.push(10);
    node.push(100);
    node.push(1000);

    node.insert(4, 3);
    println!("{}", node.stringify());

    println!("{}", node.find(3));
    println!("{}", node.find(4));

    node.prepend(5);
    println!("{}", node.stringify());

    println!("insert 6 to pos 1:");
    node.insert_v2(1, 6);
    println!("{}", node.stringify());

    println!("insert 7 to pos 2:");
    node.insert_v2(2, 7);
    println!("{}", node.stringify());

    println!("insert 8 to pos 0:");
    node.insert_v2(0, 8);
    println!("{}", node.stringify());

    let mut clone_node = node.clone();

    clone_node.insert_v2(0, 10);
    node.insert_v2(1, 9);
    println!("{}", clone_node.stringify());
    println!("{}", node.stringify());

    clone_node.insert_v2(0, 10);
    node.insert_v2(1, 9);
    println!("{}", clone_node.stringify());
    println!("{}", node.stringify());
}
