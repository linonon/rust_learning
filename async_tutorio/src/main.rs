use async_tutorio::TimerFuture;
use futures::{
    future::{BoxFuture, FutureExt},
    task::{waker_ref, ArcWake},
};
use std::{
    future::Future,
    sync::{mpsc, Arc, Mutex},
    task, thread,
    time::Duration,
};

//  從 receivers 裡接受 tasks 並運行.
struct Executor {
    ready_queue: mpsc::Receiver<Arc<Task>>,
}

// spawn 新的 futures 到 task channel 裡面去.
struct Spawner {
    task_sender: mpsc::SyncSender<Arc<Task>>,
}

impl Spawner {
    fn spawn(&self, future: impl Future<Output = ()> + Send + 'static) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        println!(
            "[{:?}] 將 future 組成 Task, 放入 Channel...",
            thread::current().id()
        );
        self.task_sender.send(task).expect("too mamy tasks queued")
    }
}

/// 一個 Future 可以重新安排自己以便被一個 `Excutor` 來進行 poll.
struct Task {
    /// 正在進行中的 Future, 他應該被推向完成.
    ///
    /// `Mutex`對於正確性來說不是必要的, 因為我們同時只有一個線程在執行任務
    /// 儘管如此, Rust 不夠聰明, 它無法知道 `future` 只由一個線程來修改
    /// 所以我們需要使用`Mutex`來保證線程安全.
    /// 生成版本的執行者不需要這個, 可以使用 `UnsafeCell` 來代替.
    future: Mutex<Option<BoxFuture<'static, ()>>>,

    /// 能把任務本身放回任務隊列的處理器
    task_sender: mpsc::SyncSender<Arc<Task>>,
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        // 通過將該任務發送回任務 channel 來實現 `wake`
        // 以便他將會被執行者再次進行 poll.
        println!("[{:?}] wake_by_ref...", thread::current().id());
        let cloned = arc_self.clone();
        arc_self
            .task_sender
            .send(cloned)
            .expect("too many tasks queued");
    }
}

/// 生成 Executor and Spawner (含發送端和接收端)...",
fn new_executor_and_spawner() -> (Executor, Spawner) {
    const MAX_QUEUED_TASKS: usize = 10000;
    let (task_sender, ready_queue) = mpsc::sync_channel(MAX_QUEUED_TASKS);
    println!(
        "[{:?}] 生成 Executor and Spawner (含發送端和接收端)...",
        thread::current().id()
    );

    (Executor { ready_queue }, Spawner { task_sender })
}

impl Executor {
    pub fn run(&self) {
        println!("[{:?}] Executor running...", thread::current().id());
        while let Ok(task) = self.ready_queue.recv() {
            println!("[{:?}] 接收到任務...", thread::current().id());
            // 獲得 future, 如果它還沒有完成 (仍然是 Some),
            // 對它進行 poll, 以嘗試完成它.
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                println!("[{:?}] 从任务中取得 Future...", thread::current().id());
                // 从任務本身創建一個 `LocalWaker`
                let waker = waker_ref(&task);
                println!("[{:?}] 獲得 waker by ref...", thread::current().id());
                let context = &mut task::Context::from_waker(&*waker);
                // `BoxFuture<T>` 是
                // `Pin<Box<dyn Future<Outpu = T> + 'static + Send>>`
                // 的類型別名.
                // 我們可以通過 `Pin::as_mut` 從它獲得
                // `Pin<&mut dyn Future + Send + 'static>`.
                println!("[{:?}] 獲得 context, 準備 Poll...", thread::current().id());
                if future.as_mut().poll(context).is_pending() {
                    // 還沒對 Future 完成處理, 所以把它放回它的任務
                    // 以便在未來再次運行.
                    *future_slot = Some(future);
                    println!("[{:?}] Poll::Pending =====", thread::current().id());
                } else {
                    println!("[{:?}] Poll::Ready...", thread::current().id());
                }
            }
        }
        println!("[{:?}] Executor run 結束", thread::current().id());
    }
}

fn main() {
    let (excutor, spawner) = new_executor_and_spawner();

    spawner.spawn(async {
        println!("[{:?}] howdy", thread::current().id());

        TimerFuture::new(Duration::new(2, 0)).await;
        println!("[{:?}] done!", thread::current().id());
    });

    println!("[{:?}] Drop Spawner!", thread::current().id());
    drop(spawner);

    excutor.run();
}
