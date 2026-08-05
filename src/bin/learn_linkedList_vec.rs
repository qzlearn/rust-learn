use std::fmt;

// 节点定义 - 使用索引代替指针
#[derive(Debug, Clone)]
struct Node<T> {
    value: T,
    next: Option<usize>,  // 下一个节点的索引
    prev: Option<usize>,  // 前一个节点的索引
}


// 基于 Vec 的双向链表
pub struct LinkedList<T> {
    nodes: Vec<Node<T>>,      // 所有节点存储在这里
    head: Option<usize>,      // 头节点索引
    tail: Option<usize>,      // 尾节点索引
    free_indices: Vec<usize>, // 空闲索引列表（用于节点回收）
    length: usize,           // 实际节点数量
}


impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            nodes: Vec::new(),
            head: None,
            tail: None,
            free_indices: Vec::new(),
            length: 0,
        }
    }

    // 在头部添加元素
    pub fn push_front(&mut self, value: T) -> usize {
        let new_index = self.allocate_node();
        
        self.nodes[new_index] = Node {
            value,
            next: self.head,
            prev: None,
        };

        // 更新旧头节点的 prev 指针
        if let Some(old_head) = self.head {
            self.nodes[old_head].prev = Some(new_index);
        }

        self.head = Some(new_index);

        // 如果链表为空，同时设置尾指针
        if self.tail.is_none() {
            self.tail = Some(new_index);
        }

        self.length += 1;
        new_index
    }

    // 在尾部添加元素
    pub fn push_back(&mut self, value: T) -> usize {
        let new_index = self.allocate_node();
        
        self.nodes[new_index] = Node {
            value,
            next: None,
            prev: self.tail,
        };

        // 更新旧尾节点的 next 指针
        if let Some(old_tail) = self.tail {
            self.nodes[old_tail].next = Some(new_index);
        }

        self.tail = Some(new_index);

        // 如果链表为空，同时设置头指针
        if self.head.is_none() {
            self.head = Some(new_index);
        }

        self.length += 1;
        new_index
    }

    // 从头部移除元素
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.map(|head_index| {
            let head_node = std::mem::replace(
                &mut self.nodes[head_index],
                Node {
                    value: unsafe { std::mem::zeroed() }, // 占位，实际会被回收
                    next: None,
                    prev: None,
                }
            );

            // 更新头指针
            self.head = head_node.next;

            // 更新新头节点的 prev 指针
            if let Some(new_head) = self.head {
                self.nodes[new_head].prev = None;
            } else {
                // 链表为空，清空尾指针
                self.tail = None;
            }

            self.free_indices.push(head_index);
            self.length -= 1;

            head_node.value
        })
    }

    // 从尾部移除元素
    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.map(|tail_index| {
            let tail_node = std::mem::replace(
                &mut self.nodes[tail_index],
                Node {
                    value: unsafe { std::mem::zeroed() },
                    next: None,
                    prev: None,
                }
            );

            // 更新尾指针
            self.tail = tail_node.prev;

            // 更新新尾节点的 next 指针
            if let Some(new_tail) = self.tail {
                self.nodes[new_tail].next = None;
            } else {
                // 链表为空，清空头指针
                self.head = None;
            }

            self.free_indices.push(tail_index);
            self.length -= 1;

            tail_node.value
        })
    }

    // 在指定节点后插入
    pub fn insert_after(&mut self, node_index: usize, value: T) -> Option<usize> {
        if node_index >= self.nodes.len() || self.nodes[node_index].next.is_none() {
            return None;
        }

        let new_index = self.allocate_node();
        let next_index = self.nodes[node_index].next;

        self.nodes[new_index] = Node {
            value,
            next: next_index,
            prev: Some(node_index),
        };

        // 更新当前节点的 next
        self.nodes[node_index].next = Some(new_index);

        // 更新下一个节点的 prev
        if let Some(next_idx) = next_index {
            self.nodes[next_idx].prev = Some(new_index);
        }

        // 如果插入在尾节点之后，更新尾指针
        if Some(node_index) == self.tail {
            self.tail = Some(new_index);
        }

        self.length += 1;
        Some(new_index)
    }

    // 根据索引获取节点的不可变引用
    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.nodes.len() {
            Some(&self.nodes[index].value)
        } else {
            None
        }
    }

    // 根据索引获取节点的可变引用
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.nodes.len() {
            Some(&mut self.nodes[index].value)
        } else {
            None
        }
    }

    // 分配新节点（复用空闲索引或扩展 Vec）
    fn allocate_node(&mut self) -> usize {
        if let Some(index) = self.free_indices.pop() {
            index
        } else {
            let index = self.nodes.len();
            self.nodes.push(Node {
                value: unsafe { std::mem::zeroed() }, // 占位值
                next: None,
                prev: None,
            });
            index
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    // 获取头节点索引
    pub fn head_index(&self) -> Option<usize> {
        self.head
    }

    // 获取尾节点索引
    pub fn tail_index(&self) -> Option<usize> {
        self.tail
    }

    // 获取下一个节点索引
    pub fn next_index(&self, index: usize) -> Option<usize> {
        self.nodes.get(index).and_then(|node| node.next)
    }

    // 获取前一个节点索引
    pub fn prev_index(&self, index: usize) -> Option<usize> {
        self.nodes.get(index).and_then(|node| node.prev)
    }
}

// 实现迭代器
pub struct LinkedListIter<'a, T> {
    list: &'a LinkedList<T>,
    current: Option<usize>,
}

impl<T> LinkedList<T> {
    pub fn iter(&self) -> LinkedListIter<'_, T> {
        LinkedListIter {
            list: self,
            current: self.head,
        }
    }
}

impl<'a, T> Iterator for LinkedListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.and_then(|index| {
            let node = &self.list.nodes[index];
            self.current = node.next;
            Some(&node.value)
        })
    }
}

// 实现 Display
impl<T: fmt::Display> fmt::Display for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        
        let mut current = self.head;
        let mut first = true;
        
        while let Some(index) = current {
            if !first {
                write!(f, " <-> ")?;
            }
            write!(f, "{}", self.nodes[index].value)?;
            first = false;
            current = self.nodes[index].next;
        }
        
        write!(f, "]")
    }
}


fn main() {
  let mut list = LinkedList::new();
    
    println!("创建链表:");
    let n1 = list.push_back(1);
    list.push_back(2);
    list.push_front(0);
    list.push_back(3);
    
    println!("{}", list); // [0 <-> 1 <-> 2 <-> 3]
    println!("长度: {}", list.len());
    println!("头节点索引: {:?}", list.head_index());
    println!("尾节点索引: {:?}", list.tail_index());
    
    println!("\n遍历链表:");
    for value in list.iter() {
        print!("{} ", value); // 0 1 2 3
    }
    println!();
    
    println!("\n修改节点值:");
    if let Some(value) = list.get_mut(n1) {
        *value = 100;
    }
    println!("{}", list); // [0 <-> 100 <-> 2 <-> 3]
    
    println!("\n删除操作:");
    println!("弹出头部: {:?}", list.pop_front()); // Some(0)
    println!("弹出尾部: {:?}", list.pop_back());   // Some(3)
    println!("剩余链表: {}", list); // [100 <-> 2]
    
    println!("\n内存使用情况:");
    println!("实际节点数: {}", list.len());
    println!("向量容量: {}", list.nodes.len());
    println!("空闲索引: {:?}", list.free_indices);
}