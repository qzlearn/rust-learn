fn main(){
     let mut v = String::from("hello,");
    let r = &mut v; //可变引用
    match r {
        value => value.push_str(" world!"),
    }
    println!("{}", v);
}