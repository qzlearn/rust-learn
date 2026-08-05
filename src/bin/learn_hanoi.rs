//汉诺塔
fn hanoi(n: usize, f: char, a: char, t: char) {
  // 检查n的合法性，避免无效值导致无限递归
  if n == 0 {
      return;
  }

  if n == 1 {
      println!("move {} from {} to {}", n, f, t);
  } else {
      hanoi(n - 1, f, t, a);
      println!("move {} from {} to {}", n, f, t);
      hanoi(n - 1, a, f, t);
  }
}

fn main() {
  let n = 3; // 汉诺塔的盘子数量
  hanoi(n, 'A', 'B', 'C'); // A, B 和 C 分别代表三个柱子
}