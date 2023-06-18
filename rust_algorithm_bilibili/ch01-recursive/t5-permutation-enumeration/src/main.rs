fn main() {
    println!("Hello, world!");

    println!("input: 3");
    Solution::new(3).permutation_enumeration(0);

    println!("input: 2");
    Solution::new(2).permutation_enumeration(0);
}

struct Solution {
    n: usize,
    arr: Vec<usize>,
    vis: Vec<usize>,
}

impl Solution {
    fn new(n: usize) -> Self {
        Solution {
            arr: vec![0; n],
            vis: vec![0; n + 1], // 记录被使用过的数字
            n,
        }
    }

    // 从 1~n 这 n 个整数排成一排并打乱次序, 按字典序输出所有可能的选择方案
    fn permutation_enumeration(&mut self, i: usize) {
        let n = self.n;

        if i == n {
            self.print_one_resule();
            return;
        }

        for k in 1..=n {
            if self.vis[k] != 0 {
                continue;
            }

            self.arr[i] = k;
            self.vis[k] = 1;
            self.permutation_enumeration(i + 1);
            self.vis[k] = 0;
        }
    }

    fn print_one_resule(&mut self) {
        for i in 0..self.n {
            print!("{} ", self.arr[i])
        }
        println!()
    }
}
