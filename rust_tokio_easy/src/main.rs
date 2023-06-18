pub mod basic;
pub mod readfile;

#[tokio::main]
/// - std::thread::sleep() 会阻塞线程
/// - tokio::time::sleep() 只是放弃 CPU, 并进入任务轮训队列
/// - tokio 的 sleep() 睡眠精度是 ms
async fn main() {
    let h1 = tokio::spawn(async {
        basic::tutorial_first().await;
    });

    let h2 = tokio::spawn(async {
        readfile::tutorial_second().await;
    });

    let _ = tokio::join!(h1, h2);
}
