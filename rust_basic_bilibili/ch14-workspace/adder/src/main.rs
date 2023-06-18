use add_one;
fn main() {
    println!("Hello, world!");
    let num = 10;
    let num2 = add_one::add_one(num);
    println!("{:?}", num2);
}
