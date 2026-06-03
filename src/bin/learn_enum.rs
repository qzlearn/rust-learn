#[derive(Debug)]
enum PokerSuit {
  Clubs(u8),
  Spades(u8),
  Diamonds(char),
  Hearts(char),
}

fn print_suit(card: PokerSuit) {
  // 需要在定义 enum PokerSuit 的上面添加上 #[derive(Debug)]，否则会报 card 没有实现 Debug
  println!("{:?}",card);
  /* match card {
    PokerSuit::Clubs(value) => println!("Clubs({value})"),
    PokerSuit::Spades(value) => println!("Spades({value})"),
    PokerSuit::Diamonds(value) => println!("Diamonds('{value}')"),
    PokerSuit::Hearts(value) => println!("Hearts('{value}')"),
  } */
}
fn main(){
  let club = PokerSuit::Clubs(10);
  let heart = PokerSuit::Spades(5);
  let diamond = PokerSuit::Diamonds('A');
  let spade = PokerSuit::Hearts('K');
  print_suit(club);
  print_suit(heart);
  print_suit(diamond);
  print_suit(spade);
}
