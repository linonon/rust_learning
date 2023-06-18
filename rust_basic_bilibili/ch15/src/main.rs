use std::{cell::RefCell, ops::Deref, rc::Rc};

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}

#[test]
fn cons_list() {
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    use List::{Cons, Nil};
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    match list {
        Cons(_, _) => println!("1"),
        Nil => println!("Nil"),
    };
}

#[test]
/// # deref
/// - 在类型和 trait 在下列三種情況發生時, Rust 會執行 deref coercion:
///   - 當 T: Deref<Target=U>, 允許 &T -> &U
///   - 當 T: DerefMut<Target=U>, 允許 &mut T -> &mut U
///   - 當 T: Deref<Target=U>, 允許 &mut T -> &U
fn deref_test() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let z = MyBox::new(x);
    assert_eq!(5, *z);

    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &T {
            &self.0
        }
    }

    fn hello(name: &str) {
        println!("hello: {}", name);
    }

    hello("rust");
    let m = MyBox::new(String::from("rust"));
    // &m -> MyBox<String>
    // deref(MyBox<String>) -> &String
    // &String -> &str
    hello(&m);
}

#[test]
fn drop_test() {
    struct CustumSmartPointer {
        data: String,
    }

    impl Drop for CustumSmartPointer {
        fn drop(&mut self) {
            println!("Dropping `{}`...", self.data);
        }
    }

    let c = CustumSmartPointer {
        data: String::from("hello"),
    };
    println!("creat c: {}", c.data);
    let d = CustumSmartPointer {
        data: String::from("world"),
    };
    println!("creat d: {}", d.data);

    println!("Custum smart pointer created.")
}

#[test]
fn rc() {
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    use List::{Cons, Nil};
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let _ = Cons(3, Rc::clone(&a));
    let _ = Cons(4, Rc::clone(&a));
}

#[test]
fn rc_count() {
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    use List::{Cons, Nil};
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

#[test]
fn rccell() {
    pub trait Messager {
        fn send(&self, msg: &str);
    }

    pub struct LimitTracker<'a, T: 'a + Messager> {
        messager: &'a T,
        value: usize,
        max: usize,
    }

    impl<'a, T> LimitTracker<'a, T>
    where
        T: Messager,
    {
        pub fn new(messager: &T, max: usize) -> LimitTracker<T> {
            LimitTracker {
                messager,
                value: 0,
                max,
            }
        }

        pub fn set_value(&mut self, value: usize) {
            self.value = value;

            let max = self.max;
            let value = self.value;
            let percentage_of_max = value as f64 / max as f64;
            if percentage_of_max >= 1.0 {
                self.messager.send("Error: You are over your quota!");
            } else if percentage_of_max >= 0.9 {
                self.messager
                    .send("Urgent warning: You've used up over 90% of your quota!");
            } else if percentage_of_max >= 0.75 {
                self.messager
                    .send("Urgent warning: You've used up over 75% of your quota!");
            }
        }
    }

    struct MockMessager {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessager {
        fn new() -> MockMessager {
            MockMessager {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messager for MockMessager {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    // #[test]
    let mock_messager = MockMessager::new();
    let mut limit_tracker = LimitTracker::new(&mock_messager, 100);

    limit_tracker.set_value(80);
    assert_eq!(mock_messager.sent_messages.borrow().len(), 1);
}
