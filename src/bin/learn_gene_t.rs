fn make_pair<T>(first: T, second: T) -> (T, T) {
    (first, second)
}
fn main() {
    let (left, right) = make_pair(80, 90);
    println!("{left} {right}");
    let (left, right) = make_pair("小林", "小周");
    println!("{left}{right}");
}
