use std::{sync::Arc, time::Instant, time::Duration};
use tokio::sync::Semaphore;
use tokio::net::TcpStream;

#[tokio::main]
async fn main(){
    let start = Instant::now();
    let semaphore = Arc::new(Semaphore::new(100));
    let mut handles = vec![];

    for port in 1..=65533{
        let semaphorecl = semaphore.clone();
        let handle = tokio::spawn(async move{
            let _permit = semaphorecl.acquire_owned().await.unwrap();
            let host_port = format!("127.0.0.1:{}", port);
            let result = tokio::time::timeout(
                Duration::from_millis(5),
                TcpStream::connect(&host_port),
            )
            .await;
            match result {
                Ok(Ok(_)) => Some(port),
                _=> None,
            }
        });
        handles.push(handle);
    }

    let mut result: Vec<u16> = vec![];
    for ele in handles {
        if let Ok(Some(port)) = ele.await{
            result.push(port);
        }
    }

    result.sort();
    println!("Open ports ({} found):",result.iter().len());
    for port in result {
        println!(" {}", port);
    }
    println!("Execution Time: {:?}", start.elapsed());
}
