/// # 快乐数
/// 1. 19 = 1^2 + 9^2 = 82
/// 2. 82 = 8^2 + 2^2 = 68
/// 3. 68 = 6^2 + 8^2 = 100
/// 4. 100 = 1^2 + 0^2 + 0^2 = 1
/// 5. 1^2 = 1
fn main() {
    assert_eq!(is_happy(19), true);
    assert_eq!(is_happy(2), false);

    assert_eq!(is_happy_v2(19), true);
    assert_eq!(is_happy_v2(2), false);
}

fn is_happy(n: usize) -> bool {
    let mut n = n;
    let mut set = std::collections::HashSet::new();
    while n != 1 {
        let mut sum = 0;
        while n != 0 {
            let digit = n % 10;
            sum += digit * digit;
            n /= 10;
        }
        if set.contains(&sum) {
            return false;
        }
        set.insert(sum);
        n = sum;
    }
    true
}

fn is_happy_v2(n: usize) -> bool {
    let mut slow = n;
    let mut fast = n;

    while fast != 1 {
        slow = get_next(slow);
        fast = get_next(get_next(fast));

        if slow == fast {
            return false;
        }
    }
    true
}

fn get_next(n: usize) -> usize {
    let mut n = n;
    let mut sum = 0;
    while n != 0 {
        let digit = n % 10;
        sum += digit * digit;
        n /= 10;
    }

    sum
}
