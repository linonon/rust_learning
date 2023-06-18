fn main() {
    assert_eq!(2, f1(5, vec![2, 2, 3, 1, 2]));
    assert_eq!(4, f1(5, vec![1, 2, 3, 1, 2]));

    assert_eq!(2, f(0, &vec![2, 2, 3, 1, 2]));
    assert_eq!(4, f(0, &vec![1, 2, 3, 1, 2]));
}

// 弹簧板
// 有一个小球掉落在一连串的弹簧板上, 小球落到某一个弹簧板后, 会被弹到某一个地点, 直到小球弹到弹簧板以外的的地方.
// 假设有 n 个连续的弹簧板, 每个弹簧板占一个单位距离, a[i] 代表第 i 个弹簧板会把小球向前弹 a[i] 个单位距离.
// 比如位置 1 的弹簧板会把小球向前弹 2 个单位距离, 到达位置 3.
// 问小球弹几次才会弹出弹簧板以外的地方?

// input:
// 1. n 代表一共有 n 个弹簧板
// 2. 输入 n 个数字, 中间用空格分开, 代表每个弹簧板的弹力
// output:
// 输出一个整数, 表示小球被弹起的次数

fn f1(n: usize, power: Vec<usize>) -> usize {
    // if i + a[i] > n; return x
    let mut res = 1;
    let mut i = 0;
    for _ in 0..power.len() {
        i += power[i];
        if i >= n {
            return res;
        }
        res += 1;
    }
    0
}

// f(i): 小球从 i 开始, 被弹出弹簧板的次数.
// f(i +a[i]): 小球的下一个位置后, 被弹出弹簧板的次数.
// 所以: f(i) = f(i + a[i]) + 1
fn f(i: usize, a: &Vec<usize>) -> usize {
    // 已经出去了, 不计次数
    if i >= a.len() {
        return 0;
    }

    f(i + a[i], a) + 1
}
