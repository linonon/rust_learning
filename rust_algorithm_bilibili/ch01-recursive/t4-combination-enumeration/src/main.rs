fn main() {
    println!("Hello, world!");

    let n = 3;
    let m = 2;
    let mut arr: Vec<usize> = vec![0; m];
    f(&mut arr, 0, 1, n, m);

    let n = 5;
    let m = 3;
    let mut arr: Vec<usize> = vec![0; m];
    f(&mut arr, 0, 1, n, m);
}

// 从 1~n 这 n 个整数中随机选取 m 个, 每种方案里的数从小到大排序, 字典序输出
fn f(arr: &mut Vec<usize>, i: usize, j: usize, n: usize, m: usize) {
    if i == m {
        print_result(arr);
        return;
    }
    for k in j..=n {
        // m - i - 1: i 后剩下的格子
        // n - k: 剩下的到 n 的数字还剩几个:w:q
        if m - i - 1 > n - k {
            break;
        }
        arr[i] = k;
        f(arr, i + 1, k + 1, n, m)
    }
}

fn print_result(arr: &Vec<usize>) {
    for i in arr {
        print!("{} ", i);
    }
    println!()
}
