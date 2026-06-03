
#[derive(Debug)]
enum MyEnum{
    Foo,
    Bar
}
fn main(){
    let v = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
    let v2: Vec<_> = v.iter().filter(|x| matches!(x, MyEnum::Foo)).collect();
    println!("{:#?}",v2);
   
}

