use futures::{
    future::{Fuse, FusedFuture, FutureExt},
    pin_mut, select,
    stream::{FusedStream, Stream, StreamExt},
};

#[allow(unused)]
async fn get_new_num() -> u8 {
    5
}

#[allow(unused)]
async fn run_on_new_num(_: u8) {}

#[allow(unused)]
async fn run_loop(
    mut interval_timer: impl Stream<Item = ()> + FusedStream + Unpin,
    starting_num: u8,
) {
    let run_on_new_num_fut = run_on_new_num(starting_num).fuse();
    let get_new_num_fut = Fuse::terminated();
    pin_mut!(run_on_new_num_fut, get_new_num_fut);

    loop {
        select! {
            () = interval_timer.select_next_some() => {
                // 定時器時間已過, 如果沒有正在運行的任務
                // 開始一個新的 `get_new_num_fut`
                if get_new_num_fut.is_terminated() {
                   get_new_num_fut.set(get_new_num().fuse());
                }
            },
            new_num = get_new_num_fut => {
                // 新的數到達了 -- 開始一個新的 `run_on_new_num_fut`,
                // 丟棄舊的
                run_on_new_num_fut.set(run_on_new_num(new_num).fuse());
            },
            () = run_on_new_num_fut => {}
            complete => panic!("`interval_timer` completed unexpectedly"),
        }
    }
}

mod fut_err_handling {
    #[allow(unused)]
    fn fut_err_handling() {
        struct MyError;
        async fn foo() -> Result<(), MyError> {
            Ok(())
        }
        async fn bar() -> Result<(), MyError> {
            Ok(())
        }

        let fut = async {
            foo().await?;
            bar().await?;
            // Ok(())
            Ok::<(), MyError>(())
        };
    }
}

mod sending {

    use std::rc::Rc;
    #[derive(Default)]
    struct NotSend(Rc<()>);

    #[allow(unused)]
    async fn bar() {}
    #[allow(unused)]
    async fn foo() {
        // ❌ not temporary
        // let x = NotSend::default();

        // ✅ call it temporarily
        NotSend::default();

        // ✅ call it in another field
        {
            let x = NotSend::default();
        }
        bar().await
    }

    #[allow(unused)]
    fn require_send(_: impl Send) {}

    #[allow(unused)]
    fn test() {
        require_send(foo());
    }
}
