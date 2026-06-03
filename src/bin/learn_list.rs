use crate::List::*;

enum List {
    // Cons: 链表中包含有值的节点，节点是元组类型，第一个元素是节点的值，第二个元素是指向下一个节点的指针
    Cons(u32, Box<List>),
    // Nil: 链表中的最后一个节点，用于说明链表的结束
    Nil,
}

// 为枚举实现一些方法
impl List {
    // 创建空的链表
    fn new() -> List {
        // 因为没有节点，所以直接返回 Nil 节点
        // 枚举成员 Nil 的类型是 List
        Nil
    }

    // 在老的链表前面新增一个节点，并返回新的链表
    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    // 返回链表的长度
    fn len(&self) -> u32 {
        match *self {
            // 这里我们不能拿走 tail 的所有权，因此需要获取它的引用
            Cons(_, ref tail) => 1 + tail.len(),
            // 空链表的长度为 0
            Nil => 0
        }
    }

    // 返回链表的字符串表现形式，用于打印输出
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // 递归生成字符串
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }

    // 向链表中间插入数据
    fn insert_at(self, elem: u32, index: u32) -> List{
      match (self, index) {
          (list, 0) => Cons(elem, Box::new(list)),
          //递归处理
          (Cons(head,tail), idx) => {
            Cons(head, Box::new(tail.insert_at(elem, idx - 1)))
          }
          (Nil, _) => Nil,
      }
    }
    // 向链表后面插入数据
    fn append(self, elem: u32) -> List{
      match self {
          // 如果尾节点，将新节点插入这里
          Nil => Cons(elem, Box::new(Nil)),
          // 对尾节点递归操作
          Cons(head, tail) => Cons(head, Box::new(tail.append(elem))),
      }
    }
}

fn main() {
    // 创建一个新的链表(也是空的)
    let mut list = List::new();

    // 添加一些元素
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);
    for i in 1..100 {
        list = list.prepend(i);
    }

    // 打印列表的当前状态
    println!("链表的长度是: {}", list.len());
    println!("{}", list.stringify());

    // 在链表中间插入元素
    list = list.insert_at(99, 1); // 在索引 1 的位置插入 99
    println!("插入后链表的长度是: {}", list.len());
    println!("{}", list.stringify());

    // 在链表末尾追加元素
    list = list.append(42);
    println!("追加后链表的长度是: {}", list.len());
    println!("{}", list.stringify());
}