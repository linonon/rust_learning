fn main() {
    // 获得第一个参数
    let args: Vec<String> = std::env::args().collect();
    println!("{}", eat_peach(args[1].parse::<i32>().unwrap()));
}

// 路飞吃桃, 买了 n 个桃子, 每天吃一半多一个, 吃了 m 天只剩一个桃子, 请问第一天买了多少个桃子?
fn eat_peach(m: i32) -> i32 {
    if m == 2 {
        return 4;
    }
    // day m-2, n/2-1 = 4, n = 10
    // day m-1, n/2-1 = 1, n = 4
    // day m,   1
    //
    // eat_peach(2) = 2*(eat_peach(1)+1)
    // eat_peach(m) = 2*(eat_peach(m-1)+1)

    return 2 * (eat_peach(m - 1) + 1);
}
