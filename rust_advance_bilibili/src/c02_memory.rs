#[test]
fn test_memory() {
    let x = 42;
    let y = 43;
    let var1 = &x;
    let mut var2 = &x;
    var2 = &y;
}

#[test]
fn test_string() {
    let pw = "string";
    let is_string = is_string(pw);
    println!("is_string: {is_string}");

    fn is_string<T: AsRef<str>>(password: T) -> bool {
        password.as_ref().len() > 5
    }

    // fn is_string<T: Into<String>>(password: T) -> bool {
    //     password.into().len() > 5
    // }
}

#[test]
/// # Heap 交互机制
/// - Heap 上面的变量必须通过指针访问
///   ```
///   let a = Box::new(40);
///   let b = 60;
///
///   let c = a + *b; // c = 100
///   ```
/// - 当 box 被丢弃时, 内存就被释放
/// - 如果忘记释放 heap 内存, 就会导致内存泄漏
/// - 有时候你就想让内存泄漏
///   - 例如有一个只读的配置, 整个程序都需要访问他. 就可以把它
///     分配在 heap 上, 通过`Box::leak`得到一个`'static`引用
///     从而显式的让其进行泄露
fn heap() {}
