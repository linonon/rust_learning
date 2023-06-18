use std::{future::Future, time::Duration};

pub async fn tutorial_first() {
    println!("Hello, tutorial_first!");

    let h1 = tokio::spawn(async {
        let _file1_contents = read_from_file1().await;
    });

    let h2 = tokio::spawn(async {
        let _file2_contents = read_from_file2().await;
    });

    let _ = tokio::join!(h1, h2);
}

// async fn read_from_file1() -> String {
//     tokio::time::sleep(Duration::new(4, 0)).await;
//     println!("{:?}", "Processing file 1");
//     String::from("Hello, 1 1 1 1")
// }

// async fn read_from_file2() -> String {
//     // sleep(Duration::new(1, 0)).await;
//     tokio::time::sleep(Duration::new(1, 0)).await;
//     println!("{:?}", "Processing file 2");
//     String::from("Hello,  2 2 2 ")
// }

fn read_from_file1() -> impl Future<Output = String> {
    async {
        tokio::time::sleep(Duration::new(4, 0)).await;
        println!("{:?}", "Processing file 1");
        String::from("Hello, basic 1 1 1 1")
    }
}

fn read_from_file2() -> impl Future<Output = String> {
    async {
        tokio::time::sleep(Duration::new(1, 0)).await;
        println!("{:?}", "Processing file 2");
        String::from("Hello, basic  2 2 2 ")
    }
}
