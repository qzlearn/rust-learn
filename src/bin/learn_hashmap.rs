use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();
    scores.insert("小林", 80);
    scores.insert("小周", 90);
    match scores.get("小林") {
        Some(score) => println!("小林:{score}"),
        None => println!("没有找到这名学生"),
    }
}
