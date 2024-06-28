fn main() {
    println!("Hello, world!");
}

#[allow(dead_code)]
fn gcd(mut a: u64, mut b: u64) -> u64 {
    assert!(a != 0 && b != 0);
    while b != 0 {
        if b < a {
            let t = a;
            a = b;
            b = t;
        }
        b = b % a;
    }
    a
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(3 * 7 * 9, 3 * 11), 3);
}

#[allow(dead_code)]
fn gcd_recursive(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd_recursive(b, a % b)
    }
}

#[test]
fn test_gcd_recursive() {
    assert_eq!(gcd_recursive(14, 15), 1);

    assert_eq!(gcd_recursive(3 * 7 * 9, 3 * 11), 3);
}
