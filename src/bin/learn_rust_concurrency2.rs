use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::net::TcpStream;
use tokio::sync::{mpsc, Mutex};

#[tokio::main]
async fn main() {
    let start = Instant::now();

    let (ports_tx, ports_rx) = mpsc::channel::<u16>(100);
    let ports_rx = Arc::new(Mutex::new(ports_rx));
    let (results_tx, mut results_rx) = mpsc::channel::<u16>(100);

    // 发送端口（对应 Go 的 goroutine）
    tokio::spawn(async move {
        for port in 1..=65535u16 {
            ports_tx.send(port).await.unwrap();
        }
    });

    // 启动 100 个常驻 worker（对应 Go 的 worker 函数）
    for _ in 0..100 {
        let rx = ports_rx.clone();
        let tx = results_tx.clone();
        tokio::spawn(async move {
            loop {
                let port = {
                    let mut rx = rx.lock().await;
                    rx.recv().await
                };
                match port {
                    Some(port) => {
                        let addr = format!("127.0.0.1:{}", port);
                        if let Ok(Ok(_)) = tokio::time::timeout(
                            Duration::from_millis(5),
                            TcpStream::connect(&addr),
                        )
                        .await
                        {
                            tx.send(port).await.ok();
                        }
                    }
                    None => break, // channel 关闭，worker 退出
                }
            }
        });
    }
    drop(results_tx);

    println!("Scanning 127.0.0.1");

    // 收集结果
    let mut opened = Vec::new();
    while let Some(port) = results_rx.recv().await {
        opened.push(port);
    }

    opened.sort();

    println!("Open ports ({} found):", opened.len());
    for port in &opened {
        println!("  {}", port);
    }
    println!("Execution Time: {:?}", start.elapsed());
}
