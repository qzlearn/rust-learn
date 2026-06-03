fn hanoi_t(num:usize, f:&str, t:&str,a:&str){
    if num == 1 {
      println!("Move disk {} from {} to {}", num, f, t);
      return;
    }

    hanoi_t(num -1, f, a, t);
    println!("Move disk {} from {} to {}", num, f, t);
    hanoi_t(num -1, a, t, f);
}

fn main(){
  let num:usize = 10;
  println!("start num:{num}");
  hanoi_t(num, "A", "C", "B");
  println!("complete");
}