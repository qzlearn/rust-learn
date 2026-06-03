//测试读取一行输入
fn main(){
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    match s.trim() {
        "foo" => println!("is foo"),
        _ => println!("{}", s)
    }
}