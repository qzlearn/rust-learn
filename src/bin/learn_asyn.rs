use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};

use tokio::join;

pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

/// Shared state between the future and the waiting thread
struct SharedState {
    /// Whether or not the sleep time has elapsed
    completed: bool,

    /// The waker for the task that `TimerFuture` is running on.
    /// The thread can use this after setting `completed = true` to tell
    /// `TimerFuture`'s task to wake up, see that `completed = true`, and
    /// move forward.
    waker: Option<Waker>,
}

//TimerFuture 承诺满足 Future 的契约
impl Future for TimerFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let thread_id = thread::current().id();
        // Look at the shared state to see if the timer has already completed.
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            println!("[poll] 线程={:?} | state=completed | → Ready", thread_id);
            Poll::Ready(())
        } else {
            // Set waker so that the thread can wake up the current task
            // when the timer has completed, ensuring that the future is polled
            // again and sees that `completed = true`.
            //
            // It's tempting to do this once rather than repeatedly cloning
            // the waker each time. However, the `TimerFuture` can move between
            // tasks on the executor, which could cause a stale waker pointing
            // to the wrong task, preventing `TimerFuture` from waking up
            // correctly.
            //
            // N.B. it's possible to check for this using the `Waker::will_wake`
            // function, but we omit that here to keep things simple.
            shared_state.waker = Some(cx.waker().clone());
            println!("[poll] 线程={:?} | state=pending | → Pending", thread_id);
            Poll::Pending
        }
    }
}

// / 为类型实现自己的方法
impl TimerFuture {
    /// Create a new `TimerFuture` which will complete after the provided
    /// timeout.
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));

        // Spawn the new thread
        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {  // ← 专门 spawn 一个新线程做计时
            let thread_id = thread::current().id();
            println!("[timer] 线程={:?} | 开始休眠", thread_id);
            thread::sleep(duration);  //   这个线程只负责睡觉
            let mut shared_state = thread_shared_state.lock().unwrap();
            // Signal that the timer has completed and wake up the last
            // task on which the future was polled, if one exists.
            shared_state.completed = true;  //   睡醒了通知 tokio
            println!("[timer] 线程={:?} | 休眠结束，唤醒 waker", thread_id);
            if let Some(waker) = shared_state.waker.take() {
                waker.wake()
            }
        });

        TimerFuture { shared_state }
    }
}

//#[tokio::main]
//async fn main() {
//    println!("两个计时器同时启动...");
    
    //
    // TimerFuture::new() 被调用
    // │
    // ├─► tokio worker 线程（ThreadId 1）
    // │     负责运行 async 代码
    // │     调用 poll() → 打印 [poll] 日志
    // │
    // └─► thread::spawn 新线程（ThreadId 14/15）
    //       负责计时 thread::sleep()
    //       睡醒后打印 [timer] 日志，调用 waker.wake()
    //
//    let (_, _) = join!(
//        async {
//            TimerFuture::new(Duration::from_secs(2)).await;
//            println!("计时器A:2秒到了");
//        },
//        async {
//            TimerFuture::new(Duration::from_secs(1)).await;
//            println!("计时器B:1秒到了");
//        }
//    );
//    
//    println!("全部完成！");
//}

// #[tokio::main] 宏展开的样子
fn main() {
    // 1. 创建 tokio Runtime（就是执行器）
    let runtime = tokio::runtime::Runtime::new().unwrap();
    
    // 2. block_on 对应 #[tokio::main] 版本里的：
    //
    //    #[tokio::main]
    //    async fn main() {          ← 这个 async fn main 就是下面的 async { } 块
    //        tokio::join!(          ← 里面的逻辑原封不动搬进来
    //            ...
    //        );
    //    }
    //
    //    宏把 async fn main 的函数体提取出来，
    //    包进 block_on(async { }) 里交给执行器驱动
    runtime.block_on(async {
        tokio::join!(
            async {
                TimerFuture::new(Duration::from_secs(2)).await;
                println!("计时器A:2秒到了");
            },
            async {
                TimerFuture::new(Duration::from_secs(1)).await;
                println!("计时器B:1秒到了");
            }
        );
    });
    
    // 3. block_on 返回说明 Future 完成，main 退出
}