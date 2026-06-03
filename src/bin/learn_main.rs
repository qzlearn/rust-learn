




use std::ops::Add;

fn main(){
    let a = 11f64;
    let b = 1.1f64;
    // 10 1.0999999999999992
    println!("{} {}", a / b, a % b);
    println!("{} {}", a.div_euclid(b), a.rem_euclid(b));
    // 0.30000000000000004
    println!("{}", 0.1 + 0.2);

    print!("{}", 0.2.add(0.1));

    let s = S(5);
    // 使用迭代器
    for i in s {
        println!("{}", i);
    }
}

struct S(usize);
// 为 S 实现迭代器
impl Iterator for S {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }

        let res = Some(self.0);
        self.0 -= 1;
        res
    }
}
