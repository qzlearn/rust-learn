use std::{thread, time::Duration};


///做两次对照：
///原样运行。 两行线程 ID 相同，但后台任务能在 main 等待期间执行。说明一条线程就能让多个任务并发推进，不需要并行。
///把等待换成 thread::sleep(Duration::from_millis(200));。 这次 main 等完了 会先打印，后台任务才有机会执行。因为唯一负责执行这些任务的线程被睡眠调用占住了。
///再把宏改回你项目里的 #[tokio::main]：默认多线程运行时下，main 仍由主线程推进，但 spawn 出来的任务可以在 Tokio 工作线程上执行。这就是前面那个“主线程在等，到底谁在跑？”的区别。
///#[tokio::main(flavor = "current_thread")]
#[tokio::main]
async fn main() {
    let bg = tokio::spawn(async {
        println!("后台任务：{:?}", thread::current().id());
    });

    println!("main 开始等待：{:?}", thread::current().id());
    //tokio::time::sleep(Duration::from_millis(200)).await;
    thread::sleep(Duration::from_millis(200));
    println!("main 等完了");

    bg.await.unwrap();
}
