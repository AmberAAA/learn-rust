enum Coin {
  Gold,
  Silver,
  Copper,
}

fn value_if_coin (coin: Coin) -> u8 {
  match coin {
    Coin::Gold => 100,
    Coin::Silver => 10,
    Coin::Copper => 1
  }
}