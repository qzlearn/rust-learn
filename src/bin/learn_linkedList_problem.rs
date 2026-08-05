use std::time::Instant;

// 有问题的节点定义 - 会导致递归 drop 和栈溢出
struct ProblematicNode {
    value: i32,
    next: Option<Box<ProblematicNode>>,
}

impl ProblematicNode {
    fn new(value: i32) -> Self {
        ProblematicNode { value, next: None }
    }
}

// 有问题的链表实现
struct ProblematicList {
    head: Option<Box<ProblematicNode>>,
}

impl ProblematicList {
    fn new() -> Self {
        ProblematicList { head: None }
    }
    
    fn push(&mut self, value: i32) {
        let new_node = Box::new(ProblematicNode {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }
    
    fn create_big_list() -> Self {
        let mut list = ProblematicList::new();
        for i in 0..1_000_000 {
            list.push(i);
        }
        list
    }
}

fn main() {
    println!("=== 有问题的实现（会导致栈溢出）===");
    
    let start = Instant::now();
    
    // 这一行会导致栈溢出！
    // let list = ProblematicList::create_big_list();
    
    // 让我们创建一个稍小的列表来演示问题
    let mut small_list = ProblematicList::new();
    for i in 0..10000 {
        small_list.push(i);
    }
    println!("创建了小列表，准备 drop...");
    // 即使是 10000 个节点，在某些系统上也可能导致栈溢出
    
    let duration = start.elapsed();
    println!("耗时: {:?}", duration);
    
    // 当 small_list 离开作用域时，会递归 drop，可能导致栈溢出
}