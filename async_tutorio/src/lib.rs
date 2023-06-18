pub mod httpweb;
pub mod pinning;
pub mod select;

use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};

pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

/// 在 Future 和等待的線程間共享狀態
#[derive(Clone, Debug)]
struct SharedState {
    /// 睡眠時間是否已經過完
    complete: bool,
    /// 'TimerFuture' 所運行的任務的 Waker
    /// 在設置`completed = true` 之後, 線程可以使用它來告訴
    /// 'TimerFuture' 的任務可以喚醒, 看到`completed = true` 並前進
    waker: Option<Waker>,
}

impl Future for TimerFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.complete {
            Poll::Ready(())
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl TimerFuture {
    pub fn new(duration: Duration) -> TimerFuture {
        let shared_state = Arc::new(Mutex::new(SharedState {
            complete: false,
            waker: None,
        }));

        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            println!(
                "[{:?}] TimerFuture 生成新的線程並開始睡眠...",
                thread::current().id()
            );
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();

            shared_state.complete = true;
            if let Some(waker) = shared_state.waker.take() {
                println!(
                    "[{:?}] TimerFuture 新線程獲得 waker, 並進行 wake()...",
                    thread::current().id()
                );
                waker.wake()
            } else {
                println!(
                    "[{:?}] TimerFuture 新線程沒獲得 waker...",
                    thread::current().id()
                );
            }
        });

        println!("[{:?}] 返回新的 TimerFuture...", thread::current().id());
        TimerFuture { shared_state }
    }
}
