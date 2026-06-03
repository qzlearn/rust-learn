fn main(){
  let x:i8 = 1;
  let y:i16 = 127;
  if let Ok(y_converted) = i8::try_from(y) {
    // 使用 checked_add 来检查是否会发生溢出
    let z = x.checked_add(y_converted);

    match z {
        Some(result) => println!("{}", result),
        None => println!("溢出发生！"),
    }
  } else {
    println!("y 的值超出 i8 的范围！");
  }
}