
fn main() {
    let s = String::from("Hello, world!");
    println!("The String is {} bytes large.", size_of_val(&s));
    println!("It contains the text: {}", s);
    let mut s = String::from("Hello, world!");
    s.push_str("Hello, world!");
    println!("{}", s);
}
