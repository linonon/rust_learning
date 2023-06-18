#[cfg(test)]
struct T1 {
    a: String,
    b: *const String,
}

#[cfg(test)]
impl T1 {
    fn new(txt: &str) -> Self {
        Self {
            a: String::from(txt),
            b: std::ptr::null(),
        }
    }

    fn init(&mut self) {
        let self_ref = &self.a;
        self.b = self_ref;
    }

    fn a(&self) -> &str {
        &self.a
    }

    fn b(&self) -> &String {
        assert!(
            !self.b.is_null(),
            "Test::b called without Test:: init being called first"
        );

        unsafe { &*(self.b) }
    }
}

#[test]
#[cfg(test)]
fn test() {
    let mut test1 = T1::new("test1");
    test1.init();
    let mut test2 = T1::new("test2");
    test2.init();

    println!("a: {}, b: {}", test1.a(), test1.b());

    std::mem::swap(&mut test1, &mut test2);

    println!("test1: a: {}, b: {}", test1.a(), test1.b());
    println!("test2: a: {}, b: {}", test2.a(), test2.b());
}

#[cfg(test)]
use {std::marker::PhantomPinned, std::pin::Pin};

#[cfg(test)]
struct T2 {
    a: String,
    b: *const String,
    _marker: PhantomPinned,
}

#[cfg(test)]
impl T2 {
    fn new(txt: &str) -> Self {
        Self {
            a: String::from(txt),
            b: std::ptr::null(),
            _marker: PhantomPinned,
        }
    }

    fn init(self: Pin<&mut Self>) {
        let self_ptr: *const String = &self.a;
        let this = unsafe { self.get_unchecked_mut() };
        this.b = self_ptr;
    }

    fn a(self: Pin<&Self>) -> &str {
        &self.get_ref().a
    }

    fn b(&self) -> &String {
        assert!(
            !self.b.is_null(),
            "Test::b called without Test:: init being called first"
        );

        unsafe { &*(self.b) }
    }
}

#[test]
fn test2() {
    let mut test1 = T2::new("test1");
    let mut test1 = unsafe { Pin::new_unchecked(&mut test1) };
    T2::init(test1.as_mut());

    let mut test2 = T2::new("test2");
    let mut test2 = unsafe { Pin::new_unchecked(&mut test2) };
    T2::init(test2.as_mut());

    println!("a: {}, b: {}", T2::a(test1.as_ref()), test1.b());

    // ↓  The type system prevent us from moving the data
    // std::mem::swap(test1.get_mut(), test2.get_mut());

    println!("test1: a: {}, b: {}", T2::a(test1.as_ref()), test1.b());
    println!("test2: a: {}, b: {}", T2::a(test2.as_ref()), test2.b());
}
