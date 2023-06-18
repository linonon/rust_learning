use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

pub struct ReadFileFuture {}

impl Future for ReadFileFuture {
    type Output = String;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("Tokio! Stop polling me");
        _cx.waker().wake_by_ref();
        Poll::Ready(String::from("Hello, there from file 1"))
    }
}

pub async fn tutorial_second() {
    println!("Hello reading file!");

    let h1 = tokio::spawn(async {
        let future1 = ReadFileFuture {};
        future1.await
    });

    let h2 = tokio::spawn(async {
        let file2_contents = read_from_file2().await;
        println!("{:?}", file2_contents);
    });

    let _ = tokio::join!(h1, h2);
}

fn read_from_file2() -> impl Future<Output = String> {
    async {
        tokio::time::sleep(Duration::new(1, 0)).await;
        println!("{:?}", "Processing file 2");
        String::from("Hello,  2 2 2 ")
    }
}
